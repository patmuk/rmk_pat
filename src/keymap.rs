use embassy_time::Duration;
use rmk::action::KeyAction;
use rmk::combo::Combo;
use rmk::config::CombosConfig;
use rmk::heapless::Vec;

use rmk::keycode::ModifierCombination;
// use rmk::{a, k, layer, mo};
use rmk::{a, k, layer, mt};
pub(crate) const COL: usize = 10;
pub(crate) const ROW: usize = 4;
pub(crate) const NUM_LAYER: usize = 2;
#[rustfmt::skip]
pub fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
let lcmd = ModifierCombination::new_from(false, true, false, false, false);
let lopt = ModifierCombination::new_from(false, false, true,false, false);
let lsft = ModifierCombination::new_from(false, false, false, true, false);
let lctl = ModifierCombination::new_from(false, false, false, false, true);
let rcmd = ModifierCombination::new_from(true, true, false, false, false);
let ropt = ModifierCombination::new_from(true, false, true,false, false);
let rsft = ModifierCombination::new_from(true, false, false, true, false);
let rctl = ModifierCombination::new_from(true, false, false, false, true);
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
 │ fn  ⌃  ⌥  ⌘  ⇧   │ │ ⇧  ⌘  ⌥  ⌃  fn │
 │  -  -  -  -  -   │ │ -  -  -  -  -  │
 ╰──────╮ NUM  SYM  │ │ CRD CMD ╭──────╯
        ╰───────────╯ ╰─────────╯         
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
// - HRM => Done, but `fn`-key doesn't exist
// - hold (layer switch)
// - combos -> need macros
// - CHORDS -> need macros
// - Repeat Function
//╭──────┬────────────┬────────────┬────────────┬────────────╮╭────────────────┬────────────┬────────────┬───────────┬───────╮
  [ k!(W),   k!(F),       k!(M),       k!(P),        k!(V),       k!(Quote),       k!(Dot),     k!(G),      k!(J),     k!(Z)],
//├──────┼────────────┼────────────┼────────────┼────────────┤├────────────────┼────────────┼────────────┼───────────┼───────┤
  [ k!(R), mt!(S,lctl), mt!(N,lopt), mt!(T,lcmd), mt!(B,lsft),  mt!(Comma,rsft), mt!(A,rcmd), mt!(E,ropt), mt!(I,rctl), k!(H)],
//├──────┼────────────┼────────────┼────────────┼────────────┤├────────────────┼────────────┼────────────┼───────────┼───────┤
  [ k!(X), k!(C),       k!(L),       k!(D),       k!(Slash),    k!(Minus),       k!(U),       k!(O),       k!(Y),      k!(K)],            
//╰──────┴────────────┴────────────╮                         ││                             ╭────────────┴───────────┴───────╯
   [a!(No),a!(No),a!(No),           k!(Backspace), k!(Again),   k!(Space),       k!(Enter),             a!(No),a!(No),a!(No)]
//                                 ╰────────────┴────────────╯╰────────────────┴────────────╯
        ]),
        layer!([
            [k!(Grave), k!(F1), k!(F2), k!(F3), k!(F4), k!(F5), k!(F6), k!(F7), k!(F8), k!(Delete)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [k!(CapsLock), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), k!(UP)]
        ]),
    ]
}

pub(crate) fn get_combos() -> CombosConfig {
    CombosConfig {
        combos: Vec::from_slice(&[Combo::new([k!(W), k!(F)], k!(Q), Some(0))])
            .expect("Some combo is not valid"),
        timeout: Duration::from_millis(50),
    }
}
