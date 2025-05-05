use rmk::action::{Action, KeyAction};
use rmk::fork::{Fork, StateBits};
use rmk::hid_state::{HidModifiers, HidMouseButtons};
use rmk::keycode::{KeyCode, ModifierCombination};
use rmk::light::LedIndicator;

/// modifiers
pub(crate) const LCMD: ModifierCombination =
    ModifierCombination::new_from(false, true, false, false, false);
pub(crate) const LOPT: ModifierCombination =
    ModifierCombination::new_from(false, false, true, false, false);
pub(crate) const LSFT: ModifierCombination =
    ModifierCombination::new_from(false, false, false, true, false);
pub(crate) const LCTL: ModifierCombination =
    ModifierCombination::new_from(false, false, false, false, true);
pub(crate) const RCMD: ModifierCombination =
    ModifierCombination::new_from(true, true, false, false, false);
pub(crate) const ROPT: ModifierCombination =
    ModifierCombination::new_from(true, false, true, false, false);
pub(crate) const RSFT: ModifierCombination =
    ModifierCombination::new_from(true, false, false, true, false);
pub(crate) const RCTL: ModifierCombination =
    ModifierCombination::new_from(true, false, false, false, true);

/// lt! but using layer names
pub(crate) const fn lt(layer: u8, key: KeyCode) -> KeyAction {
    KeyAction::LayerTapHold(Action::Key(key), layer)
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
            HidModifiers::new_from(false, true, false, false, false, false, false, false),
            LedIndicator::default(),
            HidMouseButtons::default(),
        ),
        StateBits::default(),
        HidModifiers::default(),
        false,
    )
}
