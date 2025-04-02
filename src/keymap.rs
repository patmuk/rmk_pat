use rmk::action::KeyAction;
// use rmk::{a, k, layer, mo};
use rmk::{a, k, layer};
pub(crate) const COL: usize = 10;
pub(crate) const ROW: usize = 4;
pub(crate) const NUM_LAYER: usize = 2;
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
//╭──────┬──────┬───────┬──────────────┬──────────╮╭──────────┬──────────┬──────┬──────┬───────╮
  [ k!(W), k!(F), k!(M),  k!(P),         k!(V),      k!(Quote), k!(Dot),   k!(G), k!(J), k!(Z)],
//├──────┼──────┼───────┼──────────────┼──────────┤├──────────┼──────────┼──────┼──────┼───────┤
  [ k!(R), k!(S), k!(N),  k!(T),         k!(B),      k!(Comma), k!(A),     k!(E), k!(I), k!(H)],
//├──────┼──────┼───────┼──────────────┼──────────┤├──────────┼──────────┼──────┼──────┼───────┤
  [ k!(X), k!(C), k!(L),  k!(D),         k!(Slash),  k!(Minus), k!(U),     k!(O), k!(Y), k!(K)],            
//╰──────┴──────┴───────╮                         ││                     ╭──────┴──────┴───────╯
   [a!(No),a!(No),a!(No), k!(Backspace), k!(Again),  k!(Space), k!(Enter), a!(No),a!(No),a!(No)]
//                      ╰──────────────┴──────────╯╰──────────┴──────────╯

        ]),
        layer!([
            [k!(Grave), k!(F1), k!(F2), k!(F3), k!(F4), k!(F5), k!(F6), k!(F7), k!(F8), k!(Delete)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [k!(CapsLock), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), k!(UP)]
        ]),
    ]
}
