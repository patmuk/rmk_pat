// use rmk::hid:: _state::{HidModifiers, HidMouseButtons};
use core::ops::BitOr;

use rmk::fork::{Fork, StateBits};
use rmk::types::action::{Action, KeyAction, MorseProfile};
use rmk::types::keycode::KeyCode;
use rmk::types::led_indicator::LedIndicator;
use rmk::types::modifier::ModifierCombination;
use rmk::types::mouse_button::MouseButtons;

/// modifiers
pub(crate) const LCMD: ModifierCombination = ModifierCombination::LGUI;
pub(crate) const LOPT: ModifierCombination = ModifierCombination::LALT;
pub(crate) const LSFT: ModifierCombination = ModifierCombination::LSHIFT;
pub(crate) const LCTL: ModifierCombination = ModifierCombination::LCTRL;
pub(crate) const RCMD: ModifierCombination = ModifierCombination::RGUI;
pub(crate) const ROPT: ModifierCombination = ModifierCombination::RALT;
pub(crate) const RSFT: ModifierCombination = ModifierCombination::RSHIFT;
pub(crate) const RCTL: ModifierCombination = ModifierCombination::RCTRL;

/// lt! but using layer names
pub(crate) const fn lt(layer: u8, key: KeyCode) -> KeyAction {
    KeyAction::TapHold(
        Action::Key(key),
        Action::LayerOn(layer),
        MorseProfile::const_default(),
    )
}

/// HRM for keys that already carry an implicit modifier (shifted symbols, AltGr glyphs, …).
/// Tap fires the key together with `tap_mod`; hold fires `hold_mod` alone.
pub(crate) const fn wmt(
    tap_key: KeyCode,
    tap_mod: ModifierCombination,
    hold_mod: ModifierCombination,
) -> KeyAction {
    KeyAction::TapHold(
        Action::KeyWithModifier(tap_key, tap_mod),
        Action::Modifier(hold_mod),
        MorseProfile::const_default(),
    )
}

// shortcut to define alternative outputs to shift
// e.g.: . -> :
// fork_alternative_shift(mt!(Dot, RSFT), shifted!(Semicolon))
//
// Matches EITHER left-shift or right-shift in the active modifier set, so the
// fork fires regardless of which hand sources the shift (left HRM, right HRM,
// outer-shift combo via osm!(LSFT), CRD-layer osm, etc.).
pub(crate) fn fork_alternative_shift(trigger: KeyAction, alternative: KeyAction) -> Fork {
    Fork::new(
        trigger,
        trigger,
        alternative,
        StateBits::new_from(
            ModifierCombination::LSHIFT.bitor(ModifierCombination::RSHIFT),
            LedIndicator::default(),
            MouseButtons::default(),
        ),
        StateBits::default(),
        ModifierCombination::default(),
        false,
    )
}
