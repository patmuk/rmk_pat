use rmk::heapless::Vec;
use rmk::keyboard_macros::{MacroOperation, define_macro_sequences};
use rmk::types::keycode::HidKeyCode;

// use rmk::MACRO_SPACE_SIZE;

const MACRO_SPACE_SIZE: usize = 256;

pub(crate) fn get_macro_sequences() -> [u8; MACRO_SPACE_SIZE] {
    define_macro_sequences(&[
        // qu
        Vec::from_slice(&[
            // Tap doesn't ignore modifiers (like SHIFT), Text ignores them
            MacroOperation::Tap(HidKeyCode::Q),
            MacroOperation::Text(HidKeyCode::U, false),
        ])
        .expect("too many elements"),
        // ä
        Vec::from_slice(&[
            MacroOperation::Press(HidKeyCode::LAlt),
            MacroOperation::Tap(HidKeyCode::U),
            MacroOperation::Release(HidKeyCode::LAlt),
            MacroOperation::Tap(HidKeyCode::A),
        ])
        .expect("too many elements"),
        // ö
        Vec::from_slice(&[
            MacroOperation::Press(HidKeyCode::LAlt),
            MacroOperation::Tap(HidKeyCode::U),
            MacroOperation::Release(HidKeyCode::LAlt),
            MacroOperation::Tap(HidKeyCode::O),
        ])
        .expect("too many elements"),
        // ü
        Vec::from_slice(&[
            MacroOperation::Press(HidKeyCode::LAlt),
            MacroOperation::Tap(HidKeyCode::U),
            MacroOperation::Release(HidKeyCode::LAlt),
            MacroOperation::Tap(HidKeyCode::U),
        ])
        .expect("too many elements"),
        // sch
        Vec::from_slice(&[
            MacroOperation::Tap(HidKeyCode::S),
            MacroOperation::Text(HidKeyCode::C, false),
            MacroOperation::Text(HidKeyCode::H, false),
        ])
        .expect("too many elements"),
    ])
}
