#![no_std]
#![no_main]

mod vial;
#[macro_use]
mod macros;
mod keymap;

use defmt::{info, unwrap};
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Input, Output};
use embassy_nrf::interrupt::{self, InterruptExt};
use embassy_nrf::mode::Async;
use embassy_nrf::peripherals::{RNG, SAADC, USBD};
use embassy_nrf::saadc::{self, AnyInput, Input as _, Saadc};
use embassy_nrf::usb::Driver;
use embassy_nrf::usb::vbus_detect::HardwareVbusDetect;
use embassy_nrf::{Peri, bind_interrupts, rng, usb};
use nrf_mpsl::Flash;
use nrf_sdc::mpsl::MultiprotocolServiceLayer;
use nrf_sdc::{self as sdc, mpsl};
use panic_probe as _;
use rand_chacha::ChaCha12Rng;
use rand_core::SeedableRng;
use rmk::ble::{BleTransport, build_ble_stack};
use rmk::config::{
    BehaviorConfig, BleBatteryConfig, DeviceConfig, Hand, KeyboardMacrosConfig, MorsesConfig,
    PositionalConfig, RmkConfig, StorageConfig, VialConfig,
};
use rmk::debounce::default_debouncer::DefaultDebouncer;
use rmk::event::*;
use rmk::futures::future::join;
use rmk::host::HostService;
use rmk::input_device::adc::{AnalogEventType, NrfAdc};
use rmk::input_device::battery::BatteryProcessor;

use rmk::keyboard::Keyboard;
use rmk::matrix::Matrix;
use rmk::processor::builtin::wpm::WpmProcessor;
use rmk::split::ble::central::scan_peripherals;
use rmk::split::central::run_peripheral_manager;
use rmk::usb::UsbTransport;
use rmk::{HostResources, KeymapData, initialize_keymap_and_storage, run_all};
use static_cell::StaticCell;
use vial::{VIAL_KEYBOARD_DEF, VIAL_KEYBOARD_ID};

bind_interrupts!(struct Irqs {
    USBD => usb::InterruptHandler<USBD>;
    SAADC => saadc::InterruptHandler;
    RNG => rng::InterruptHandler<RNG>;
    EGU0_SWI0 => nrf_sdc::mpsl::LowPrioInterruptHandler;
    CLOCK_POWER => nrf_sdc::mpsl::ClockInterruptHandler, usb::vbus_detect::InterruptHandler;
    RADIO => nrf_sdc::mpsl::HighPrioInterruptHandler;
    TIMER0 => nrf_sdc::mpsl::HighPrioInterruptHandler;
    RTC0 => nrf_sdc::mpsl::HighPrioInterruptHandler;
});

#[embassy_executor::task]
async fn mpsl_task(mpsl: &'static MultiprotocolServiceLayer<'static>) -> ! {
    mpsl.run().await
}

/// How many outgoing L2CAP buffers per link
const L2CAP_TXQ: u8 = 3;

/// How many incoming L2CAP buffers per link
const L2CAP_RXQ: u8 = 3;

/// Size of L2CAP packets
const L2CAP_MTU: usize = 251;

fn build_sdc<'d, const N: usize>(
    p: nrf_sdc::Peripherals<'d>,
    rng: &'d mut rng::Rng<Async>,
    mpsl: &'d MultiprotocolServiceLayer,
    mem: &'d mut sdc::Mem<N>,
) -> Result<nrf_sdc::SoftdeviceController<'d>, nrf_sdc::Error> {
    sdc::Builder::new()?
        .support_scan()
        .support_central()
        .support_adv()
        .support_peripheral()
        .support_dle_peripheral()
        .support_dle_central()
        .support_phy_update_central()
        .support_phy_update_peripheral()
        .support_le_2m_phy()
        .central_count(1)?
        .peripheral_count(1)?
        .buffer_cfg(L2CAP_MTU as u16, L2CAP_MTU as u16, L2CAP_TXQ, L2CAP_RXQ)?
        .build(p, rng, mpsl, mem)
}

/// Initializes the SAADC peripheral in single-ended mode on the given pin.
fn init_adc(adc_pin: AnyInput, adc: Peri<'static, SAADC>) -> Saadc<'static, 1> {
    // Then we initialize the ADC. We are only using one channel in this example.
    let config = saadc::Config::default();
    let channel_cfg = saadc::ChannelConfig::single_ended(adc_pin.degrade_saadc());
    interrupt::SAADC.set_priority(interrupt::Priority::P3);

    saadc::Saadc::new(adc, Irqs, config, [channel_cfg])
}

fn ble_addr() -> [u8; 6] {
    let ficr = embassy_nrf::pac::FICR;
    let high = u64::from(ficr.deviceid(1).read());
    let addr = high << 32 | u64::from(ficr.deviceid(0).read());
    let addr = addr | 0x0000_c000_0000_0000;
    unwrap!(addr.to_le_bytes()[..6].try_into())
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Hello RMK BLE!");
    // Initialize the peripherals and nrf-sdc controller
    let mut nrf_config = embassy_nrf::config::Config::default();
    nrf_config.dcdc.reg0_voltage = Some(embassy_nrf::config::Reg0Voltage::_3V3);
    nrf_config.dcdc.reg0 = true;
    nrf_config.dcdc.reg1 = true;
    let p = embassy_nrf::init(nrf_config);
    let mpsl_p =
        mpsl::Peripherals::new(p.RTC0, p.TIMER0, p.TEMP, p.PPI_CH19, p.PPI_CH30, p.PPI_CH31);
    let lfclk_cfg = mpsl::raw::mpsl_clock_lfclk_cfg_t {
        source: mpsl::raw::MPSL_CLOCK_LF_SRC_RC as u8,
        rc_ctiv: mpsl::raw::MPSL_RECOMMENDED_RC_CTIV as u8,
        rc_temp_ctiv: mpsl::raw::MPSL_RECOMMENDED_RC_TEMP_CTIV as u8,
        accuracy_ppm: mpsl::raw::MPSL_DEFAULT_CLOCK_ACCURACY_PPM as u16,
        skip_wait_lfclk_started: mpsl::raw::MPSL_DEFAULT_SKIP_WAIT_LFCLK_STARTED != 0,
    };
    static MPSL: StaticCell<MultiprotocolServiceLayer> = StaticCell::new();
    static SESSION_MEM: StaticCell<mpsl::SessionMem<1>> = StaticCell::new();
    let mpsl = MPSL.init(unwrap!(mpsl::MultiprotocolServiceLayer::with_timeslots(
        mpsl_p,
        Irqs,
        lfclk_cfg,
        SESSION_MEM.init(mpsl::SessionMem::new())
    )));
    spawner.spawn(mpsl_task(&*mpsl).unwrap());
    let sdc_p = sdc::Peripherals::new(
        p.PPI_CH17, p.PPI_CH18, p.PPI_CH20, p.PPI_CH21, p.PPI_CH22, p.PPI_CH23, p.PPI_CH24,
        p.PPI_CH25, p.PPI_CH26, p.PPI_CH27, p.PPI_CH28, p.PPI_CH29,
    );
    let mut rng = rng::Rng::new(p.RNG, Irqs);
    let mut rng_gen = ChaCha12Rng::from_rng(&mut rng).unwrap();
    let mut sdc_mem = sdc::Mem::<8192>::new();
    let sdc = unwrap!(build_sdc(sdc_p, &mut rng, mpsl, &mut sdc_mem));
    let mut host_resources = HostResources::new();
    let stack = build_ble_stack(sdc, ble_addr(), &mut rng_gen, &mut host_resources).await;

    // Initialize usb driver
    let driver = Driver::new(p.USBD, Irqs, HardwareVbusDetect::new(Irqs));

    // Initialize flash
    let flash = Flash::take(mpsl, p.NVMC);

    // Initialize IO Pins
    // nice!nano pins (https://nicekeyboards.com/docs/nice-nano/pinout-schematic) to
    // rows2cols (https://docs.splitkb.com/product-guides/aurora-series/schematics/aurora-sweep)
    // output_pin =>   >|   => input_pin
    //                 ↑
    //               diode(be aware of it's direction)
    // left pins
    // P0_09 = col0
    // P0_22 = col1
    // P0_24 = col2
    // P1_00 = col3
    // P0_11 = col4
    // P0_02 = row0
    // P0_29 = row1
    // P1_15 = row2
    // P1_13 = row3
    let (col_pins, row_pins) = config_matrix_pins_nrf!(
        peripherals: p,
        input:  [P0_11, P1_00, P0_24, P0_22, P0_09],
        output: [P0_02, P0_29, P1_15, P1_13]);

    // Initialize the ADC.
    // We are only using one channel for detecting battery level
    let adc_pin = p.P0_05.degrade_saadc();
    let is_charging_pin = Input::new(p.P0_07, embassy_nrf::gpio::Pull::Up);
    let saadc = init_adc(adc_pin, p.SAADC);
    // Wait for ADC calibration.
    saadc.calibrate().await;

    // Keyboard config
    let keyboard_device_config = DeviceConfig {
        vid: 0x4c4b,
        pid: 0x4643,
        manufacturer: "RMK",
        product_name: "Pats AuroraSweep",
        serial_number: "vial:f64c2b3c:000001",
    };
    // unlock_keys = keys to unlock vial config : row=0 col=0 and row=1 col=1
    let vial_config = VialConfig::new(VIAL_KEYBOARD_ID, VIAL_KEYBOARD_DEF, &[(0, 0), (1, 1)]);
    let ble_battery_config = BleBatteryConfig::new(Some(is_charging_pin), true, None, false);
    let storage_config = StorageConfig {
        start_addr: 0xA0000, // FIXME: use 0x70000 after we can build without softdevice controller
        num_sectors: 6,
        clear_storage: true,
        ..Default::default()
    };
    let rmk_config = RmkConfig {
        device_config: keyboard_device_config,
        vial_config,
        ble_battery_config,
        storage_config,
    };

    // Initialze keyboard stuffs
    // Initialize the storage and keymap
    let mut keymap_data = KeymapData::new(keymap::get_default_keymap());
    let mut behavior_config = BehaviorConfig {
        combo: keymap::get_combos(),
        morse: MorsesConfig {
            enable_flow_tap: true,
            ..Default::default()
        },
        fork: keymap::get_forks(),
        keyboard_macros: KeyboardMacrosConfig::new(keymap::keymap_macros::get_macro_sequences()),
        ..Default::default()
    };
    let key_config = PositionalConfig::new(
        [[
            Hand::Left,
            Hand::Left,
            Hand::Left,
            Hand::Left,
            Hand::Left,
            Hand::Right,
            Hand::Right,
            Hand::Right,
            Hand::Right,
            Hand::Right,
        ]; 4],
    );
    let (keymap, mut storage) = initialize_keymap_and_storage(
        &mut keymap_data,
        flash,
        &storage_config,
        &mut behavior_config,
        &key_config,
    )
    .await;

    // Initialize the matrix and keyboard
    let debouncer = DefaultDebouncer::new();
    let mut matrix = Matrix::<_, _, _, 4, 5, false>::new(row_pins, col_pins, debouncer);
    // let mut matrix = TestMatrix::<ROW, COL>::new();
    let mut keyboard = Keyboard::new(&keymap);
    let mut host_service = HostService::new(&keymap, &rmk_config);

    // Read peripheral address from storage
    let peripheral_addrs = storage.read_peripheral_addresses::<1>().await;

    // Initialize the encoder processor
    let mut adc_device = NrfAdc::new(
        saadc,
        [AnalogEventType::Battery],
        embassy_time::Duration::from_secs(12),
        None,
    );
    let mut batt_proc = BatteryProcessor::new(2000, 2806);

    // Initialize the controllers
    // let mut capslock_led = KeyboardIndicatorProcessor::new(
    //     Output::new(
    //         p.P0_00,
    //         embassy_nrf::gpio::Level::Low,
    //         embassy_nrf::gpio::OutputDrive::Standard,
    //     ),
    //     false,
    //     rmk::types::led_indicator::LedIndicatorType::CapsLock,
    // );

    // Peripheral battery monitor controller
    // This controller subscribes to PeripheralBatteryEvent events
    // and logs the battery level of each peripheral
    use rmk::event::PeripheralBatteryEvent;
    use rmk::macros::processor;

    #[processor(subscribe = [PeripheralBatteryEvent, BatteryStatusEvent, LayerChangeEvent])]
    struct PeripheralBatteryMonitor {}

    impl PeripheralBatteryMonitor {
        async fn on_peripheral_battery_event(&mut self, event: PeripheralBatteryEvent) {
            info!("Peripheral {} battery status: {:?}", event.id, event);
        }
        async fn on_battery_status_event(&mut self, event: BatteryStatusEvent) {
            info!("Central battery status: {:?}", event);
        }
        async fn on_layer_change_event(&mut self, event: LayerChangeEvent) {
            info!("Layer changed to: {}", event.0);
        }
    }

    let mut peripheral_battery_monitor = PeripheralBatteryMonitor {};

    let mut usb_transport = UsbTransport::new(driver, rmk_config.device_config);
    let mut ble_transport = BleTransport::new(&stack, rmk_config).await;
    let mut wpm_processor = WpmProcessor::new();

    // Start
    join(
        run_all!(
            matrix,
            adc_device,
            storage,
            usb_transport,
            ble_transport,
            wpm_processor,
            batt_proc,
            keyboard,
            host_service,
            peripheral_battery_monitor
        ),
        join(
            run_peripheral_manager::<4, 5, 0, 5, _>(0, &peripheral_addrs, &stack),
            scan_peripherals(&stack, &peripheral_addrs),
        ),
    )
    .await;
}
