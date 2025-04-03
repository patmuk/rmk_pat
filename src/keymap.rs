use core::ops::BitOr;

use embassy_time::Duration;
use rmk::action::{Action, KeyAction};
use rmk::combo::Combo;
use rmk::config::CombosConfig;
use rmk::heapless::Vec;

use rmk::keycode::{KeyCode, ModifierCombination};
// use rmk::{a, k, layer, mo};
use rmk::{a, k, layer, lt, mt, osl, osm, shifted, th, wm};
pub(crate) const COL: usize = 10;
pub(crate) const ROW: usize = 4;
pub(crate) const NUM_LAYER: usize = 3;
enum ModK {
    Lcmd,
    Lopt,
    Lsft,
    Lctl,
    Rcmd,
    Ropt,
    Rctl,
    Rsft,
}
fn mod_k(mod_key: ModK) -> ModifierCombination {
    match mod_key {
        ModK::Lcmd => ModifierCombination::new_from(false, true, false, false, false),
        ModK::Lopt => ModifierCombination::new_from(false, false, true, false, false),
        ModK::Lsft => ModifierCombination::new_from(false, false, false, true, false),
        ModK::Lctl => ModifierCombination::new_from(false, false, false, false, true),
        ModK::Rcmd => ModifierCombination::new_from(true, true, false, false, false),
        ModK::Ropt => ModifierCombination::new_from(true, false, true, false, false),
        ModK::Rsft => ModifierCombination::new_from(true, false, false, true, false),
        ModK::Rctl => ModifierCombination::new_from(true, false, false, false, true),
    }
}
// Alpha_shitf layer HRM
fn mt_sft(key: KeyCode) -> KeyAction {
    KeyAction::LayerTapHold(Action::Key(key), ALPHA_SFT)
}
/// word wise backspace
fn wrd_bsp() -> KeyAction {
    KeyAction::WithModifier(
        Action::Key(rmk::keycode::KeyCode::Backspace),
        mod_k(ModK::Lopt),
    )
}
const ALPHA: u8 = 0;
const ALPHA_SFT: u8 = 1;
const NUM: u8 = 2;
const SYM: u8 = 3;
const CMD: u8 = 4;

#[rustfmt::skip]
pub fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
  //layers
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
 ╰────────╮ ⌫W REP│ │ ␣  ⏎   ╭──────╯
          ╰───────╯ ╰────────╯         
 hold  
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
// DONE
// - alternative shift
//   - , => ;
//   - . => :
//   - / => |
// TODO
// - hold (layer switch)
//   - tri-layers
//   - mo_NUM + mo_SYM = mo CMD
//   - mo_CHORD + mo_CMD = mo FUN
// - combos -> need macros
//   - W+F = Qu
//   - S+E = ß
//    - A+E = Ä
//    - U+E = Ü
//    - O+E = Ö
//    - S+H = sch
//    - Lsft+Rsft = CapsWord
//    - wrd_bsp+space = Backspace
//    - left outer thumb + right outer thumb = shift in CMD layer
//  == need RMK extension: 
// - CHORDS -> need macros
// - Repeat Function
// - HRM => `fn`-key (doesn't exist) (called 'globe' key, need to set vendor ID to apple)
//╭──────┬────────────┬────────────┬────────────┬────────────╮╭────────────────┬────────────┬────────────┬───────────┬───────╮
  [ k!(W),   k!(F),       k!(M),       k!(P),        k!(V),       k!(Quote),       k!(Comma),     k!(G),      k!(J),     k!(Z)],
//├──────┼────────────┼────────────┼────────────┼────────────┤├────────────────┼────────────┼────────────┼───────────┼───────┤
// R|Lsft, S|Lctl       N|Lopt        T|Lcmd        B|sft         .|sft           A|Rcmd        E|Ropt      I|Rctl     H|Rsft
  [ mt!(R, mod_k(ModK::Lsft)), mt!(S,mod_k(ModK::Lctl)), mt!(N,mod_k(ModK::Lopt)), mt!(T,mod_k(ModK::Lcmd)), mt_sft(KeyCode::B), mt_sft(KeyCode::Dot), mt!(A,mod_k(ModK::Rcmd)), mt!(E,mod_k(ModK::Ropt)), mt!(I,mod_k(ModK::Rctl)), mt!(H, mod_k(ModK::Rsft))],
//├──────┼────────────┼────────────┼────────────┼────────────┤├────────────────┼────────────┼────────────┼───────────┼───────┤
  [ k!(X), k!(C),       k!(L),       k!(D),       k!(Slash),    k!(Minus),       k!(U),       k!(O),       k!(Y),      k!(K)],            
//╰──────┴────────────┴────────────╮                         ││                             ╭────────────┴───────────┴───────╯
   [a!(No),a!(No),a!(No),            wrd_bsp(),   k!(Again),   k!(Space),       k!(Enter),             a!(No),a!(No),a!(No)]
//                                 ╰────────────┴────────────╯╰────────────────┴────────────╯
        ]),
        layer!([// Shifted Alpha (Base)
//╭──────┬────────────┬────────────┬────────────┬────────────╮╭────────────────┬────────────┬────────────┬───────────┬───────╮
  [ shifted!(W),   shifted!(F),       shifted!(M),       shifted!(P),        shifted!(V),       shifted!(Quote),       k!(Semicolon),     shifted!(G),      shifted!(J),     shifted!(Z)],
//├──────┼────────────┼────────────┼────────────┼────────────┤├────────────────┼────────────┼────────────┼───────────┼───────┤
  [ shifted!(R), shifted!(S), shifted!(N), shifted!(T), shifted!(B),  shifted!(Semicolon), shifted!(A), shifted!(E), shifted!(I), shifted!(H)],
//├──────┼────────────┼────────────┼────────────┼────────────┤├────────────────┼────────────┼────────────┼───────────┼───────┤
  [ shifted!(X), shifted!(C),       shifted!(L),       shifted!(D),       shifted!(Backslash),    shifted!(Minus),       shifted!(U),       shifted!(O),       shifted!(Y),      shifted!(K)],            
//╰──────┴────────────┴────────────╮                         ││                             ╭────────────┴───────────┴───────╯
   [a!(No),a!(No),a!(No),            k!(Backspace), k!(Again),   k!(Space),       k!(Enter),             a!(No),a!(No),a!(No)]
//                                 ╰────────────┴────────────╯╰────────────────┴────────────╯
        ]),
        layer!([
          //layer for VIAL modifications
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
            Combo::new([k!(W), k!(F)], shifted!(Q), Some(ALPHA_SFT)),
            //CapsW -> doesn't work
            // Combo::new([k!(Dot), k!(B)], osm!(mod_k(ModK::Lsft)), None),
            // Combo::new([k!(R), k!(H)], osm!(mod_k(ModK::Lsft)), None),
            //Backspace behavior
            Combo::new([wrd_bsp(), k!(Space)], k!(Backspace), None),
        ])
        .expect("Some combo is not valid"),
        timeout: Duration::from_millis(50),
    }
}
