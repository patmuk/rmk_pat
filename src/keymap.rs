use embassy_time::Duration;
use rmk::combo::{Combo, ComboConfig};
use rmk::config::{CombosConfig, ForksConfig};
use rmk::heapless::Vec;
use rmk::types::action::KeyAction;

use rmk::{a, k, macros, mt, osm, shifted, wm};

use core::ops::BitOr;
mod general_helpers;
use general_helpers::*;
#[macro_use]
mod key_aliases;
use key_aliases::__;
use key_aliases::XX;
pub(crate) mod keymap_macros;

pub(crate) const COL: usize = 10;
pub(crate) const ROW: usize = 4;
pub(crate) const NUM_LAYER: usize = 6;

/// layer names
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
    ╭───────────────╮ ╭─────────────────╮
    │ Qu            │ │                 │
    │ / \           │ │                 │
    │ W  F  _  _  _ │ │_  _     _  _  _ │
    │ _  ß -_--_--_ │ │_- E -Ä- A  _  _ │
    │               │ │   │ \           │
    │               │ │   Ü  Ö          │
    │               │ │   │    \        │
    │ _  _  _  _  _ │ │_  U     O  _  _ │
    ╰────────╮_-W⌫-_│ │_  _ ╭───────────╯
             ╰──────╯ ╰─────╯
  */
  [// Alpha (Base)
    // - combos
    //   - S+E = ß
    //    - A+E = Ä
    //    - U+E = Ü
    //    - O+E = Ö
    //    - S+H = sch
    //   - LSFT+RSFT = CapsWord
    // TODO
    // - hold (layer switch)
    //   - tri-layers
    //   - mo_NUM + mo_SYM = mo CMD
    //   - mo_CHORD + mo_CMD = mo FUN
    //    - left outer thumb + right outer thumb = shift in CMD layer
    //  == need RMK extension:
    // - HRM => `fn`-key (doesn't exist) (called 'globe' key, need to set vendor ID to apple)
    //╭──────────┬───────────┬────────────┬───────────┬──────────────╮╭────────────┬───────────┬───────────┬───────────┬────────────╮
      [ k!(W),     k!(F),      k!(M),       k!(P),      k!(V),          K!("'"),     K!(","),    k!(G),      k!(J),      k!(Z)     ],
    //├──────────┼───────────┼────────────┼───────────┼──────────────┤├────────────┼───────────┼───────────┼───────────┼────────────┤
      [K!("R|l⇧"), K!("S|l⌃"), K!("N|l⎇"), K!("T|l⌘"), K!("B|l⇧"),     K!(".|r⇧"),  K!("A|r⌘"), K!("E|r⎇"), K!("I|r⌃"), K!("H|r⇧")],
    //├──────────┼───────────┼────────────┼───────────┼──────────────┤├────────────┼───────────┼───────────┼───────────┼────────────┤
      [ k!(X),     k!(C),      k!(L),       k!(D),      K!("/"),        K!("-"),     k!(U),      k!(O),      k!(Y),      k!(K)     ],
    //╰──────────┴───────────┴────────────╮                          ││                         ╭──────────┴───────────┴────────────╯
      [ XX,        XX,         XX,        K!("⌫|🅛NUM"),K!("🔁|🅛SYM"),  K!("␣|🅛⇉"), K!("⏎|🅛⌘"),  XX,        XX,         XX        ]
    //                                    ╰───────────┴──────────────╯╰────────────┴────────────╯
  ],
  [// NUM
    // TODO thumb keys
    // TODO change to unicode symmbols once Macros are working
    //╭─────────┬───────────┬────────────┬───────────┬───────────╮╭───────────┬───────────┬───────────┬───────────┬───────────╮
      [K!("*|/"), K!("9"),    K!("8"),     K!("7"),    K!(","),     K!("'"),     K!("!"),    __,         K!("°"),    K!("∑")    ],
    //├─────────┼───────────┼────────────┼───────────┼───────────┤├───────────┼───────────┼───────────┼───────────┼───────────┤
      [K!("+|-|l⇧"), K!("3|l⌃"), K!("2|l⎇"), K!("1|l⌘"), K!("0|l⇧"),  K!("§|r⇧"), K!("%|r⌘"), K!("≤|r⎇"), K!("≥|r⌃"), K!("#|r⇧")],
    //├─────────┼───────────┼────────────┼───────────┼───────────┤├───────────┼───────────┼───────────┼───────────┼───────────┤
      [K!("=|^"), K!("6"),    K!("5"),     K!("4"),    K!("."),     K!("_"),     K!("µ"),    K!("±"),    K!("≈"),    K!("≠")    ],
    //╰─────────┴───────────┴────────────╮                       ││                 ╭────────┴────────┴─────────╯
      [__,        __,         __,           __,        __,          __,      __,      __,      __,      __     ]
    //                                   ╰───────────┴───────────╯╰────────┴────────╯
  ],
  [//SYM
    //╭─────────────┬──────────────┬──────────────┬──────────────┬─────────────╮╭────────────┬────────────┬────────────┬────────────┬────────────╮
      [ K!("←"),     K!("↑"),       K!("↓"),       K!("→"),       K!("^"),       K!("`"),     K!("?"),     K!("▼"),     K!("▼"),     K!("▼")     ],
    //├─────────────┼──────────────┼──────────────┼──────────────┼─────────────┤├────────────┼────────────┼────────────┼────────────┼────────────┤
      [K!("{|}|l⇧"), K!("[|]|l⌃"), K!("(|)|l⎇"), K!("<|>|l⌘"), K!("F⌫|l⇧"),   K!("!|r⇧"), K!("@|r⌘"), K!("=|r⎇"), K!("&|r⌃"), K!("#|r⇧")],
    //├─────────────┼──────────────┼──────────────┼──────────────┼─────────────┤├────────────┼────────────┼────────────┼────────────┼────────────┤
      [__,           __,            K!("~"),       K!("$"),       K!("∖"),       K!("-"),     K!("€"),     __,          __,          __         ],
    //╰─────────┴──────────┴──────────╮                    ││                ╭────────┴────────┴─────────╯
      [__,        __,        __,        K!("W⌫"),  __,       __,      __,      __,      __,      __     ]
    //                                 ╰─────────┴─────────╯╰───────┴────────╯
  ],
  [//CMD
    // TODO think of Window mid
    //╭──────────────┬──────────────┬──────────────┬──────────────┬───────────────╮╭───────────────┬───────────────┬─────────┬───────────────┬───────────────╮
      [K!("🔎∗"),     K!("🔎"),       K!("↷"),       K!("↶"),       K!("⇨⧉"),       K!("🔉"),       K!("🔇"),       K!("🔊"), K!("🔅"),       K!("🔆")        ],
    //├──────────────┼──────────────┼──────────────┼──────────────┼───────────────┤├───────────────┼───────────────┼─────────┼───────────────┼───────────────┤
      [K!("✔*|l⇧"), K!("✂|l⌃"),   K!("⧉|l⎇"),   K!("📋|l⌘"),   K!("📋*|l⇧"),   K!("🖥️⬅|r⇧"), K!("⬅▢|r⌘"),  __,       K!("▢🡺|r⌃"), K!("🡺🖥️|r⇧")],
    //├──────────────┼──────────────┼──────────────┼──────────────┼───────────────┤├───────────────┼───────────────┼─────────┼───────────────┼───────────────┤
      [K!("⇤🔍"),    K!("🔎⇥"),    K!("↩"),       K!("⇥"),       K!("⇨▢"),       K!("⎋"),        K!("⏮"),        K!("⏯"),  K!("⏭"),        __             ],
    //╰─────────┴──────────┴────────╮                    ││                     ╭─────────┴──────────┴───────────╯
    [__, __, __, __, __, __, __, __, __, __]
    //                              ╰──────────┴─────────╯╰──────────┴──────────╯
    ],
  [//CRD
    //╭─────┬─────┬─────┬─────┬─────╮╭─────┬─────┬─────┬────┬─────╮
    [k!(C), __, __, __, __, __, __, __, __, __],
    //├─────┼─────┼─────┼─────┼─────┤├─────┼─────┼─────┼────┼─────┤
    [osm!(LSFT), osm!(LCTL), osm!(LOPT), osm!(LCMD), osm!(LSFT), osm!(RSFT), osm!(RCMD), osm!(ROPT), osm!(RCTL), osm!(RSFT)],
    //├─────┼─────┼─────┼─────┼─────┤├─────┼─────┼─────┼────┼─────┤
    [__, __, __, __, __, __, __, __, __, __],
    //╰──────┴────────────┴────────────╮                         ││                             ╭────────────┴───────────┴───────╯
    [__, __, __, __, __, __, __, __, __, __]
    //                                 ╰────────────┴────────────╯╰────────────────┴────────────╯
  ],
  [//layer for VIAL modifications
    [__, __, __, __, __, __, __, __, __, __],
    [__, __, __, __, __, __, __, __, __, __],
    [__, __, __, __, __, __, __, __, __, __],
    [__, __, __, __, __, __, __, __, __, __]
  ],
    ]
}

/// forks
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
            fork_alternative_shift(mt!(KpPlus, LSFT), k!(KpMinus)),
            // TODO fix, broken because of wm! ?
            // // = -> ^
            // fork_alternative_shift(k!(KpEqual), wm!(I, LOPT)),
            // { -> }
            fork_alternative_shift(shifted!(LeftBracket), shifted!(RightBracket)),
            // ( -> )
            fork_alternative_shift(shifted!(Kc9), shifted!(Kc0)),
            // < -> >
            fork_alternative_shift(shifted!(Comma), shifted!(Dot)),
        ])
        .expect("error defining forks"),
    }
}

pub(crate) fn get_combos() -> CombosConfig {
    CombosConfig {
        combos: [
            // W + F -> qu
            // use with LSFT + RSFT -> Qu
            Some(Combo::new(ComboConfig::new(
                [k!(W), k!(F)],
                macros!(0),
                Some(ALPHA),
            ))),
            // R + S -> ESC (left HRM)
            Some(Combo::new(ComboConfig::new(
                [K!("R|l⇧"), K!("S|l⌃")],
                k!(Escape),
                Some(ALPHA),
            ))),
            // I + H -> ESC (right HRM, symmetric)
            Some(Combo::new(ComboConfig::new(
                [K!("I|r⌃"), K!("H|r⇧")],
                k!(Escape),
                Some(ALPHA),
            ))),
            // CapsW
            Some(Combo::new(ComboConfig::new(
                [mt!(B, LSFT), mt!(Dot, RSFT)],
                k!(CapsWordToggle),
                None,
            ))),
            // S + E -> ß
            Some(Combo::new(ComboConfig::new(
                [K!("S|l⌃"), K!("E|r⎇")],
                wm!(S, LOPT),
                Some(ALPHA),
            ))),
            // A + E -> ä
            Some(Combo::new(ComboConfig::new(
                [K!("A|r⌘"), K!("E|r⎇")],
                macros!(1),
                Some(ALPHA),
            ))),
            // O + E -> ö
            Some(Combo::new(ComboConfig::new(
                [k!(O), K!("E|r⎇")],
                macros!(2),
                Some(ALPHA),
            ))),
            // U + E -> ü
            Some(Combo::new(ComboConfig::new(
                [k!(U), K!("E|r⎇")],
                macros!(3),
                Some(ALPHA),
            ))),
            // S + H -> sch("W⌫")
            Some(Combo::new(ComboConfig::new(
                [K!("S|l⌃"), K!("H|r⇧")],
                macros!(4),
                Some(ALPHA),
            ))),
            // left side both thumbs = ("W⌫")
            Some(Combo::new(ComboConfig::new(
                [K!("⌫|🅛NUM"), K!("🔁|🅛SYM")],
                K!("W⌫"),
                Some(ALPHA),
            ))),
            // Both outermost shift keys -> one-shot shift (per layer)
            Some(Combo::new(ComboConfig::new(
                [K!("R|l⇧"), K!("H|r⇧")],
                osm!(LSFT),
                Some(ALPHA),
            ))),
            Some(Combo::new(ComboConfig::new(
                [K!("{|}|l⇧"), K!("#|r⇧")],
                osm!(LSFT),
                Some(SYM),
            ))),
            Some(Combo::new(ComboConfig::new(
                [K!("+|-|l⇧"), K!("#|r⇧")],
                osm!(LSFT),
                Some(NUM),
            ))),
            Some(Combo::new(ComboConfig::new(
                [K!("✔*|l⇧"), K!("🡺🖥️|r⇧")],
                osm!(LSFT),
                Some(CMD),
            ))),
            None,
            None,
        ],
        timeout: Duration::from_millis(50),
    }
}
