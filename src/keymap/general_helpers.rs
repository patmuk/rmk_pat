// use rmk::hid:: _state::{HidModifiers, HidMouseButtons};
use rmk::types::action::{Action, KeyAction};
use rmk::types::fork::{Fork, StateBits};
use rmk::types::keycode::KeyCode;
use rmk::types::led_indicator::LedIndicator;
use rmk::types::modifier::ModifierCombination;
use rmk::types::morse::MorseProfile;
use rmk::types::mouse_button::MouseButtons;

/// modifiers
pub(crate) const LCMD: ModifierCombination = ModifierCombination::LGUI;
pub(crate) const LOPT: ModifierCombination = ModifierCombination::LALT;
pub(crate) const LSFT: ModifierCombination = ModifierCombination::LSHIFT;
pub(crate) const LCTL: ModifierCombination = ModifierCombination::LCTRL;
pub(crate) const RCMD: ModifierCombination = ModifierCombination::RCTRL;
pub(crate) const ROPT: ModifierCombination = ModifierCombination::RALT;
pub(crate) const RSFT: ModifierCombination = ModifierCombination::RSHIFT;
pub(crate) const RCTL: ModifierCombination = ModifierCombination::RCTRL;

// TODO fix. Morse?
/// lt! but using layer names
pub(crate) const fn lt(layer: u8, key: KeyCode) -> KeyAction {
    KeyAction::TapHold(
        Action::Key(key),
        Action::LayerOn(layer),
        MorseProfile::const_default(),
    )
}

// shortcut to define alternative outputs to shift
// e.g.: . -> :
// fork_alternative_shift(mt!(Dot, RSFT), shifted!(Semicolon))
pub(crate) fn fork_alternative_shift(trigger: KeyAction, alternative: KeyAction) -> Fork {
    Fork::new(
        trigger,
        trigger,
        alternative,
        StateBits::new_from(
            ModifierCombination::new_from(false, false, false, true, false),
            LedIndicator::default(),
            MouseButtons::default(),
        ),
        StateBits::default(),
        ModifierCombination::default(),
        false,
    )
}
