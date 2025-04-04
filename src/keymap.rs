use core::ops::BitOr;

use embassy_time::Duration;
use rmk::action::{Action, KeyAction};
use rmk::combo::Combo;
use rmk::config::{CombosConfig, ForksConfig};
use rmk::fork::{Fork, StateBits};
use rmk::heapless::Vec;

use rmk::hid_state::{HidModifiers, HidMouseButtons};
use rmk::keycode::{KeyCode, ModifierCombination};
use rmk::light::LedIndicator;
// use rmk::{a, k, layer, mo};
use rmk::{a, k, layer, lt, mo, mt, osl, osm, shifted, tg, th, wm};
pub(crate) const COL: usize = 10;
pub(crate) const ROW: usize = 4;
pub(crate) const NUM_LAYER: usize = 6;

const LCMD: ModifierCombination = ModifierCombination::new_from(false, true, false, false, false);
const LOPT: ModifierCombination = ModifierCombination::new_from(false, false, true, false, false);
const LSFT: ModifierCombination = ModifierCombination::new_from(false, false, false, true, false);
const LCTL: ModifierCombination = ModifierCombination::new_from(false, false, false, false, true);
const RCMD: ModifierCombination = ModifierCombination::new_from(true, true, false, false, false);
const ROPT: ModifierCombination = ModifierCombination::new_from(true, false, true, false, false);
const RSFT: ModifierCombination = ModifierCombination::new_from(true, false, false, true, false);
const RCTL: ModifierCombination = ModifierCombination::new_from(true, false, false, false, true);

fn lt(layer: u8, key: KeyCode) -> KeyAction {
    KeyAction::LayerTapHold(Action::Key(key), layer)
}

const ALPHA: u8 = 0;
const NUM: u8 = 1;
const SYM: u8 = 2;
const CMD: u8 = 3;
const CRD: u8 = 4;

#[rustfmt::skip]
pub fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
  [
  // Alpha (base) Layer
    /*  Based on: 34 Keys - Hands Down neu
    https://sites.google.com/alanreiser.com/handsdown/home/hands-down-neu?authuser=0
    ╭───────────────╮ ╭─────────────────╮
    │ w  f  m  p  v │ │ /  .  q  "  '  z│
    │ r  s  n  t  b │ │ ,  a  e  i  h  j│
    │ x  c  l  d  g │ │ -  u  o  y  k   │     
    ╰─────────╮,; .:│ │ ␣  ⏎╭───────────╯
              ╰─────╯ ╰─────╯         
  === Variante Pat ===
    ╭────────────────╮ ╭───────────────╮
    │ w  f  m  p  v  │ │ '  .  g  j  z │
    │ r  s  n  t  b  │ │ ,  a  e  i  h │
    │ x  c  l  d  /  │ │ -  u  o  y  k │
    ╰────────╮ ⌫ REP │ │ ␣  ⏎   ╭──────╯
              ╰───────╯ ╰────────╯         
    // ╰────────╮ ⌫W REP│ │ ␣  ⏎   ╭──────╯
    // ⌫W only possible with Macros (for lt! to work ([osm!(Lctl), k!(Backspace)])) 
  hold (HRM)
    ╭──────────────────╮ ╭────────────────╮
    │  -  -  -  -  -   │ │ -  -  -  -  -  │
    │  ⇧  ⌃  ⌥  ⌘  ⇧   │ │ ⇧  ⌘  ⌥  ⌃  ⇧  │
    │  -  -  -  -  -   │ │ -  -  -  -  -  │
    ╰──────╮ NUM  SYM  │ │ CRD CMD ╭──────╯
           ╰───────────╯ ╰─────────╯         
    (The innermost shift is a layer toggle, this shall be used for shifting the keys (so we get alternative shifted symbols). 
    Because this isn't combinable with the HRM, the outermost shift is a normal shift key for combinations with othr modifier keys.)

   Chorded Letters
    ╭──────────────╮ ╭─────────────────╮
    │ Qu           │ │                 │
    │ / \          │ │                 │
    │ W  F  _  _  _│ │_  _     _  _  _ │
    │ _  ß -_--_--_│ │_- E -Ä- A  _  _ │
    │              │ │   │ \           │
    │              │ │   Ü  Ö          │
    │              │ │   │    \        │
    │ _  _  _  _  _│ │_  U     O  _  _ │
    ╰────────╮ _  _│ │_  _ ╭───────────╯
             ╰─────╯ ╰─────╯         
  */   
  layer!([// Alpha (Base)
    // TODO
    // - WBsp: hold space and press backspace (wbsp on CRD layer)
    // - hold (layer switch)
    //   - tri-layers
    //   - mo_NUM + mo_SYM = mo CMD
    //   - mo_CHORD + mo_CMD = mo FUN
    //    - wrd_bsp+space = Backspace
    //    - left outer thumb + right outer thumb = shift in CMD layer
    // - combos -> need macros
    //   - W+F = Qu
    //   - S+E = ß
    //    - A+E = Ä
    //    - U+E = Ü
    //    - O+E = Ö
    //    - S+H = sch
    //  == need RMK extension: 
    // - CHORDS -> need macros
    // - Repeat Function
    // - HRM => `fn`-key (doesn't exist) (called 'globe' key, need to set vendor ID to apple)
    //╭──────┬────────────┬────────────┬────────────┬────────────╮╭────────────────┬────────────┬────────────┬───────────┬───────╮
      [ k!(W),   k!(F),       k!(M),       k!(P),        k!(V),       k!(Quote),       k!(Comma),     k!(G),      k!(J),     k!(Z)],
    //├──────┼────────────┼────────────┼────────────┼────────────┤├────────────────┼────────────┼────────────┼───────────┼───────┤
    // R|Lsft, S|Lctl       N|Lopt        T|Lcmd        B|sft         .|sft           A|Rcmd        E|Ropt      I|Rctl     H|Rsft
      [ mt!(R, LSFT), mt!(S,LCTL), mt!(N,LOPT), mt!(T,LCMD), mt!(B, LSFT), mt!(Dot,RSFT), mt!(A,RCMD), mt!(E,ROPT), mt!(I,RCTL), mt!(H, RSFT)],
    //├──────┼────────────┼────────────┼────────────┼────────────┤├────────────────┼────────────┼────────────┼───────────┼───────┤
      [ k!(X), k!(C),       k!(L),       k!(D),       k!(Slash),    k!(Minus),       k!(U),       k!(O),       k!(Y),      k!(K)],            
    //╰──────┴────────────┴─────────╮                            ││                             ╭────────────┴───────────┴───────╯
    //                                NUM|Backspace   SYM|Again     CRD|Space        CMD|Enter
      [a!(No),a!(No),a!(No), lt(NUM, KeyCode::Backspace),lt(SYM, KeyCode::Again), lt(CRD, KeyCode::Space), lt(CMD, KeyCode::Enter), a!(No),a!(No),a!(No)]
    //                              ╰───────────────┴────────────╯╰────────────────┴────────────╯
  ]),
  layer!([// NUM
    // TODO change to unicode symmbols once Macros are working
    // TODO alternate shifted versions
    //  -> ^ / -
    // TODO fix: +
    // TODO thumb keys
    //╭─────┬─────┬─────┬─────┬─────╮╭─────┬─────┬─────┬────┬─────╮
    //  *|/    9     8     7     ,      '     !           ˚    ∑
    [k!(KpAsterisk), k!(Kc9), k!(Kc8), k!(Kc7), k!(KpComma), k!(Quote), shifted!(Kc1), a!(No), wm!(K, ROPT), wm!(W, ROPT)],
    //├─────┼─────┼─────┼─────┼─────┤├─────┼─────┼─────┼────┼─────┤
    //  +|-    3     2     1     0      §     %     ≤     ≥    #
    [k!(KpPlus), k!(Kc3), k!(Kc2), k!(Kc1), k!(Kc0), wm!(Kc6, ROPT), shifted!(Kc5), wm!(Comma, ROPT), wm!(Dot, ROPT), shifted!(Kc3)],
    //├─────┼─────┼─────┼─────┼─────┤├─────┼─────┼─────┼────┼─────┤
    //  =|^    6     5     4     .      _     µ     ±     ≈     ≠      
    [k!(KpEqual), k!(Kc6), k!(Kc5),k!(Kc4), k!(KpDot),  shifted!(Minus), wm!(M, ROPT), wm!(Equal, ROPT.bitor(RSFT)), wm!(X, ROPT), wm!(Equal, ROPT)],
    //╰──────┴────────────┴────────────╮                         ││                             ╭────────────┴───────────┴───────╯
    [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)]
    //                                 ╰────────────┴────────────╯╰────────────────┴────────────╯
  ]),
  layer!([//SYM
    //╭─────┬─────┬─────┬─────┬─────╮╭─────┬─────┬─────┬────┬─────╮
    [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
    //├─────┼─────┼─────┼─────┼─────┤├─────┼─────┼─────┼────┼─────┤
    [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
    //├─────┼─────┼─────┼─────┼─────┤├─────┼─────┼─────┼────┼─────┤
    [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
    //╰──────┴────────────┴────────────╮                         ││                             ╭────────────┴───────────┴───────╯
    [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)]
    //                                 ╰────────────┴────────────╯╰────────────────┴────────────╯
  ]),
  layer!([//CMD
    //╭─────┬─────┬─────┬─────┬─────╮╭─────┬─────┬─────┬────┬─────╮
    [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
    //├─────┼─────┼─────┼─────┼─────┤├─────┼─────┼─────┼────┼─────┤
    [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
    //├─────┼─────┼─────┼─────┼─────┤├─────┼─────┼─────┼────┼─────┤
    [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
    //╰──────┴────────────┴────────────╮                         ││                             ╭────────────┴───────────┴───────╯
    [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)]
    //                                 ╰────────────┴────────────╯╰────────────────┴────────────╯
  ]),
  layer!([//CRD
    //╭─────┬─────┬─────┬─────┬─────╮╭─────┬─────┬─────┬────┬─────╮
    [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
    //├─────┼─────┼─────┼─────┼─────┤├─────┼─────┼─────┼────┼─────┤
    [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
    //├─────┼─────┼─────┼─────┼─────┤├─────┼─────┼─────┼────┼─────┤
    [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
    //╰──────┴────────────┴────────────╮                         ││                             ╭────────────┴───────────┴───────╯
    [a!(Transparent), a!(Transparent), wm!(Backspace, ROPT), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)]
    //                                 ╰────────────┴────────────╯╰────────────────┴────────────╯
  ]),
  layer!([//layer for VIAL modifications
    [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
    [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
    [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
    [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)]
  ]),
  ]
}

pub(crate) fn get_combos() -> CombosConfig {
    CombosConfig {
        combos: Vec::from_slice(&[
            Combo::new([k!(W), k!(F)], k!(Q), Some(ALPHA)),
            // CapsW
            Combo::new([mt!(B, LSFT), mt!(Dot, RSFT)], osm!(LSFT), None),
        ])
        .expect("Some combo is not valid"),
        timeout: Duration::from_millis(50),
    }
}

fn fork_alternative_shift(trigger: KeyAction, alternative: KeyAction) -> Fork {
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

pub(crate) fn get_forks() -> ForksConfig {
    ForksConfig {
        forks: Vec::from_slice(&[
            // . -> :
            fork_alternative_shift(mt!(Dot, RSFT), shifted!(Semicolon)),
            // , -> ,
            fork_alternative_shift(k!(Comma), k!(Semicolon)),
            // / -> |
            fork_alternative_shift(k!(Slash), shifted!(Backslash)),
            // * -> /
            fork_alternative_shift(k!(KpAsterisk), k!(KpSlash)),
            // + -> -
            fork_alternative_shift(k!(Slash), shifted!(Backslash)),
            // = -> ^
            fork_alternative_shift(k!(Slash), shifted!(Backslash)),
            // wBsp -> Bsp
            // TODO not working, because lt and ht at the same time isn't possible
            // Fork::new(
            //     lt(KeyCode::Backspace, NUM),
            //     wm!(Backspace, LOPT),
            //     k!(Backspace),
            //     StateBits::new_from(
            //         H_LSFT.bitor(H_RSFT),
            //         LedIndicator::default(),
            //         HidMouseButtons::default(),
            //     ),
            //     StateBits::default(),
            //     HidModifiers::from_bits(0b11111111),
            //     false,
            // ),
        ])
        .expect("Some fork is not valid"),
    }
}
