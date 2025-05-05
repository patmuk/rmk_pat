use rmk::heapless::Vec;
use rmk::keyboard_macros::keyboard_macro::{define_macro_sequences, MacroOperation};
// use rmk::MACRO_SPACE_SIZE;

use rmk::keycode::KeyCode;

const MACRO_SPACE_SIZE: usize = 256;

pub(crate) fn get_macro_sequences() -> [u8; MACRO_SPACE_SIZE] {
    define_macro_sequences(&[
        // qu
        Vec::from_slice(&[
            // Tap doesn't ignore modifiers (like SHIFT), Text ignores them
            MacroOperation::Tap(KeyCode::Q),
            MacroOperation::Text(KeyCode::U, false),
        ])
        .expect("too many elements"),
        // ä
        Vec::from_slice(&[
            MacroOperation::Press(KeyCode::LAlt),
            MacroOperation::Tap(KeyCode::U),
            MacroOperation::Release(KeyCode::LAlt),
            MacroOperation::Tap(KeyCode::A),
        ])
        .expect("too many elements"),
        // ö
        Vec::from_slice(&[
            MacroOperation::Press(KeyCode::LAlt),
            MacroOperation::Tap(KeyCode::U),
            MacroOperation::Release(KeyCode::LAlt),
            MacroOperation::Tap(KeyCode::O),
        ])
        .expect("too many elements"),
        // ü
        Vec::from_slice(&[
            MacroOperation::Press(KeyCode::LAlt),
            MacroOperation::Tap(KeyCode::U),
            MacroOperation::Release(KeyCode::LAlt),
            MacroOperation::Tap(KeyCode::U),
        ])
        .expect("too many elements"),
        // sch
        Vec::from_slice(&[
            MacroOperation::Tap(KeyCode::S),
            MacroOperation::Text(KeyCode::C, false),
            MacroOperation::Text(KeyCode::H, false),
        ])
        .expect("too many elements"),
    ])
}
