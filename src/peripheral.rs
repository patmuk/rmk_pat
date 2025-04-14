#![no_std]
#![no_main]

mod vial;
#[macro_use]
mod macros;
mod keymap;

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Input, Output};
use embassy_nrf::interrupt::{self, InterruptExt};
use embassy_nrf::peripherals::{RNG, SAADC, USBD};
use embassy_nrf::saadc::{self, AnyInput, Input as _, Saadc};
use embassy_nrf::{bind_interrupts, rng, usb, Peri};
use nrf_sdc::mpsl::MultiprotocolServiceLayer;
use nrf_sdc::{self as sdc, mpsl};
use rand_chacha::ChaCha12Rng;
use rand_core::SeedableRng;
use rmk::ble::trouble::build_ble_stack;
use rmk::channel::EVENT_CHANNEL;
use rmk::debounce::default_debouncer::DefaultDebouncer;
use rmk::futures::future::join;
use rmk::matrix::Matrix;
use rmk::split::peripheral::run_rmk_split_peripheral;
use rmk::{run_devices, HostResources};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

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
const L2CAP_MTU: usize = 72;

fn build_sdc<'d, const N: usize>(
    p: nrf_sdc::Peripherals<'d>,
    rng: &'d mut rng::Rng<RNG>,
    mpsl: &'d MultiprotocolServiceLayer,
    mem: &'d mut sdc::Mem<N>,
) -> Result<nrf_sdc::SoftdeviceController<'d>, nrf_sdc::Error> {
    sdc::Builder::new()?
        .support_adv()?
        .support_peripheral()?
        .peripheral_count(1)?
        .buffer_cfg(L2CAP_MTU as u8, L2CAP_MTU as u8, L2CAP_TXQ, L2CAP_RXQ)?
        .build(p, rng, mpsl, mem)
}

/// Initializes the SAADC peripheral in single-ended mode on the given pin.
fn init_adc(adc_pin: AnyInput, adc: Peri<'static, SAADC>) -> Saadc<'static, 1> {
    // Then we initialize the ADC. We are only using one channel in this example.
    let config = saadc::Config::default();
    let channel_cfg = saadc::ChannelConfig::single_ended(adc_pin.degrade_saadc());
    interrupt::SAADC.set_priority(interrupt::Priority::P3);
    let saadc = saadc::Saadc::new(adc, Irqs, config, [channel_cfg]);
    saadc
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Hello RMK BLE!");
    // Initialize the peripherals and nrf-sdc controller
    let mut nrf_config = embassy_nrf::config::Config::default();
    nrf_config.dcdc.reg0_voltage = Some(embassy_nrf::config::Reg0Voltage::_3v3);
    nrf_config.dcdc.reg0 = true;
    nrf_config.dcdc.reg1 = true;
    let p = embassy_nrf::init(nrf_config);
    let mpsl_p = mpsl::Peripherals::new(p.RTC0, p.TIMER0, p.TEMP, p.PPI_CH19, p.PPI_CH30, p.PPI_CH31);
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
    spawner.must_spawn(mpsl_task(&*mpsl));
    let sdc_p = sdc::Peripherals::new(
        p.PPI_CH17, p.PPI_CH18, p.PPI_CH20, p.PPI_CH21, p.PPI_CH22, p.PPI_CH23, p.PPI_CH24, p.PPI_CH25, p.PPI_CH26,
        p.PPI_CH27, p.PPI_CH28, p.PPI_CH29,
    );
    let mut rng = rng::Rng::new(p.RNG, Irqs);
    let mut rng_generator = ChaCha12Rng::from_rng(&mut rng).unwrap();
    let mut sdc_mem = sdc::Mem::<4096>::new();
    let sdc = unwrap!(build_sdc(sdc_p, &mut rng, mpsl, &mut sdc_mem));

    let peripheral_addr = [0x7e, 0xfe, 0x73, 0x9e, 0x66, 0xe3];
    let mut resources = HostResources::new();
    let stack = build_ble_stack(sdc, peripheral_addr, &mut rng_generator, &mut resources).await;

    // Initialize the ADC. We are only using one channel for detecting battery level
    let adc_pin = p.P0_05.degrade_saadc();
    let saadc = init_adc(adc_pin, p.SAADC);
    // Wait for ADC calibration.
    saadc.calibrate().await;

    // nice!nano pins (https://nicekeyboards.com/docs/nice-nano/pinout-schematic) to
    // rows2cols (https://docs.splitkb.com/product-guides/aurora-series/schematics/aurora-sweep)
    // right pins
    // P1_06 = col0
    // P1_04 = col1
    // P0_11 = col2
    // P1_00 = col3
    // P0_24 = col4
    // P1_13 = row0
    // P1_15 = row1
    // P0_02 = row2
    // P1_11 = row3
    let (input_pins, output_pins) = config_matrix_pins_nrf!(
        peripherals: p, 
        input:  [P1_06, P1_04, P0_11, P1_00, P0_24], 
        output: [P1_13, P1_15, P0_02, P1_11]);

    let central_addr = [0x18, 0xe2, 0x21, 0x80, 0xc0, 0xc7];


    // Initialize the peripheral matrix
    let debouncer = DefaultDebouncer::<5, 4>::new();
    let mut matrix = Matrix::<_, _, _, 5, 4>::new(input_pins, output_pins, debouncer);

    // Start
    join(
        run_devices! (
            (matrix) => EVENT_CHANNEL, // Peripheral uses EVENT_CHANNEL to send events to central
        ),
        run_rmk_split_peripheral(central_addr, &stack),
    )
    .await;
}
