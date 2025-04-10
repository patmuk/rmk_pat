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
use rmk::{a, k, layer, mt, osm, shifted, wm};
pub(crate) const COL: usize = 10;
pub(crate) const ROW: usize = 4;
pub(crate) const NUM_LAYER: usize = 6;

/// key shortcuts
macro_rules! K {
    ("␣") => {
        lt(NUM, KeyCode::Backspace)
    };
    (",") => {
        k!(Comma)
    };
    ("'") => {
        k!(Quote)
    };
    (".") => {
        k!(Dot)
    };
    ("R|l⇧") => {
        mt!(R, LSFT)
    };
    ("S|l⌃") => {
        mt!(S, LCTL)
    };
    ("N|l⎇") => {
        mt!(N, LOPT)
    };
    ("T|l⌘") => {
        mt!(T, LCMD)
    };
    ("B|l⇧") => {
        mt!(B, LSFT)
    };
    (".|r⇧") => {
        mt!(Dot, RSFT)
    };
    ("A|r⌘") => {
        mt!(A, RCMD)
    };
    ("E|r⎇") => {
        mt!(E, ROPT)
    };
    ("I|r⌃") => {
        mt!(I, RCTL)
    };
    ("H|r⇧") => {
        mt!(H, RSFT)
    };
    ("/") => {
        k!(Slash)
    };
    ("-") => {
        k!(Minus)
    };
    ("▼") => {
        a!(Transparent)
    };
    ("⌫") => {
        k!(Backspace)
    };
    ("W⌫") => {
        wm!(Backspace, ROPT)
    };
    ("⌫|🅛NUM") => {
        lt(Num, KeyCode::Backspace)
    };
    ("🔁|🅛SYM") => {
        lt(SYM, KeyCode::Again)
    };
    ("␣|🅛⇉") => {
        lt(CRD, KeyCode::Space)
    };
    ("⏎|🅛⌘") => {
        lt(CMD, KeyCode::Enter)
    };
    ("*") => {
        k!(KpAsterisk)
    };
    ("*|/") => {
        k!(KpAsterisk)
    };
    ("!") => {
        shifted!(Kc1)
    };
    ("K|r⎇") => {
        wm!(K, ROPT)
    };
    ("W|r⎇") => {
        wm!(W, ROPT)
    };
    ("9") => {
        k!(Kc9)
    };
    ("8") => {
        k!(Kc8)
    };
    ("7") => {
        k!(Kc7)
    };
    ("6") => {
        k!(Kc6)
    };
    ("5") => {
        k!(Kc5)
    };
    ("4") => {
        k!(Kc4)
    };
    ("°") => {
        wm!(Kc8, ROPT.bitor(RSFT))
    };
    ("∑") => {
        wm!(W, ROPT)
    };
    ("+|-") => {
        mt!(KpPlus, LSFT)
    };
    ("3|l⌃") => {
        mt!(Kc3, LCTL)
    };
    ("2|l⎇") => {
        mt!(Kc2, LOPT)
    };
    ("1|l⌘") => {
        mt!(Kc1, LCMD)
    };
    ("0|l⇧") => {
        mt!(Kc0, LSFT)
    };
    (".|r⇧") => {
        mt!(Dot, RSFT)
    };
    ("A|r⌘") => {
        mt!(A, RCMD)
    };
    ("E|r⎇") => {
        mt!(E, ROPT)
    };
    ("I|r⌃") => {
        mt!(I, RCTL)
    };
    ("H|r⇧") => {
        mt!(H, RSFT)
    };
    ("§") => {
        wm!(Kc6, ROPT)
    };
    ("%") => {
        shifted!(Kc5)
    };
    ("≤") => {
        wm!(Comma, ROPT)
    };
    ("≥") => {
        wm!(Dot, ROPT)
    };
    ("#") => {
        shifted!(Kc3)
    };
    ("=|^") => {
        k!(KpEqual)
    };
    ("_") => {
        shifted!(Minus)
    };
    ("µ") => {
        wm!(M, ROPT)
    };
    ("±") => {
        wm!(Equal, ROPT.bitor(RSFT))
    };
    ("≈") => {
        wm!(X, ROPT)
    };
    ("≠") => {
        wm!(Equal, ROPT)
    };
    ("←") => {
        k!(Left)
    };
    ("↑") => {
        k!(Up)
    };
    ("↓") => {
        k!(Down)
    };
    ("→") => {
        k!(Right)
    };
    ("^") => {
        wm!(I, LOPT)
    };
    ("`") => {
        k!(GraveEscape)
    };
    ("?") => {
        shifted!(Minus)
    };
    ("{|}") => {
        shifted!(LeftBracket)
    };
    ("[|]") => {
        k!(LeftBracket)
    };
    ("(|)") => {
        shifted!(Kc9)
    };
    ("<|>") => {
        shifted!(Comma)
    };
    ("F⌫") => {
        wm!(D, LCTL)
    };
    ("!") => {
        shifted!(Kc1)
    };
    ("@") => {
        shifted!(Kc2)
    };
    ("=") => {
        k!(Equal)
    };
    ("&") => {
        shifted!(Kc7)
    };
    ("#") => {
        shifted!(Kc3)
    };
    ("~") => {
        wm!(N, LOPT)
    };
    ("$") => {
        shifted!(Kc4)
    };
    ("∖") => {
        k!(Backslash)
    };
    ("-") => {
        k!(Minus)
    };
    ("€") => {
        wm!(Kc2, RSFT.bitor(ROPT))
    };
    // find all
    ("🔎∗") => {
        wm!(F, LCMD.bitor(LSFT))
    };
    // find
    ("🔎") => {
        wm!(F, LSFT)
    };
    // redo
    ("↷") => {
        wm!(Z, LCMD.bitor(LSFT))
    };
    // undo
    ("↶") => {
        wm!(Z, LCMD)
    };
    // switch App
    ("⇨⧉") => {
        wm!(Tab, LCMD)
    };
    ("🔉") => {
        k!(KbVolumeDown)
    };
    ("🔇") => {
        k!(KbMute)
    };
    ("🔊") => {
        k!(KbVolumeUp)
    };
    ("🔅") => {
        k!(BrightnessDown)
    };
    ("🔆") => {
        k!(BrightnessUp)
    };
    //├─────────┼─────┼─────┼─────┼─────────┤├──────┼──────┼─────┼────────┼───────┤
    // SelectAll  cut  copy  paste pasteHist <-space <-win  winMid  win->  space->
    // select all
    ("✔*") => {
        wm!(A, LCMD)
    };
    ("✂") => {
        wm!(X, LCMD)
    };
    ("⧉") => {
        wm!(C, LCMD)
    };
    ("📋") => {
        wm!(V, LCMD)
    };
    ("📋*") => {
        wm!(V, LCMD.bitor(LSFT))
    };
    // Switch to left desktop
    ("🖥️⬅") => {
        wm!(Left, RCTL)
    };
    // place window left
    ("⬅▢") => {
        wm!(Left, RCTL.bitor(ROPT).bitor(RSFT))
    };
    // place window right
    ("▢🡺") => {
        wm!(Right, RCTL.bitor(ROPT).bitor(RSFT))
    };
    ("🡺🖥️") => {
        wm!(Right, RCTL)
    };
    // findPrev
    ("⇤🔍") => {
        wm!(Enter, LSFT)
    };
    //fNext Enter  tab   switchWin   ESC   prev  play/pause next
    ("🔎⇥") => {
        wm!(N, LSFT)
    };
    ("↩") => {
        k!(Enter)
    };
    ("⇥") => {
        k!(Tab)
    };
    // switch windows
    ("⇨▢") => {
        wm!(Grave, LCMD)
    };
    ("⎋") => {
        k!(Escape)
    };
    ("⏮") => {
        k!(MediaPrevTrack)
    };
    ("⏯") => {
        k!(MediaPlayPause)
    };
    ("⏭") => {
        k!(MediaNextTrack)
    };
    // forward delete
    // TODO implement fn key!
    // ("⌦") => {
    //   wm!(Backspace, _fn_)
    // };
    //
    // Catch-all case to trigger compile-time error if the string doesn't match
    ($key:expr) => {
        compile_error!(concat!("Unmatched key: ", $key))
    };
}

const XX: KeyAction = a!(No);
const __: KeyAction = a!(Transparent);

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
    ╰────────╮ W⌫ _│ │_  _ ╭───────────╯
             ╰─────╯ ╰─────╯
  */   
  layer!([// Alpha (Base)
    // TODO
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
    //  - CapsW for Chord using Bsp key
    // - Repeat Function
    // - HRM => `fn`-key (doesn't exist) (called 'globe' key, need to set vendor ID to apple)
    //╭──────────┬───────────┬────────────┬───────────┬──────────────╮╭────────────┬───────────┬───────────┬───────────┬────────────╮
      [ k!(W),     k!(F),      k!(M),       k!(P),      k!(V),          K!("'"),     K!(","),    k!(G),      k!(J),      k!(Z)     ],
    //├──────────┼───────────┼────────────┼───────────┼──────────────┤├────────────┼───────────┼───────────┼───────────┼────────────┤
      [K!("R|l⇧"), K!("S|l⌃"), K!("N|l⎇"), K!("T|l⌘"), K!("B|l⇧"),     K!(".|r⇧"),  K!("A|r⌘"), K!("E|r⎇"), K!("I|r⌃"), K!("H|r⇧")],
    //├──────────┼───────────┼────────────┼───────────┼──────────────┤├────────────┼───────────┼───────────┼───────────┼────────────┤
      [ k!(X),     k!(C),      k!(L),       k!(D),      K!("/"),        K!("-"),     k!(U),      k!(O),      k!(Y),      k!(K)     ],            
    //╰──────────┴───────────┴────────────╮                          ││                         ╭──────────┴───────────┴────────────╯
      [ XX,        XX,         XX,          K!("␣"),    K!("🔁|🅛SYM"),  K!("␣|🅛⇉"), K!("⏎|🅛⌘"), XX,        XX,         XX        ]
    //                                    ╰───────────┴──────────────╯╰────────────┴────────────╯
  ]),
  layer!([// NUM
    // TODO thumb keys
    // TODO HRM right side -> doesn't work with shifted keys! Macros?
    // TODO change to unicode symmbols once Macros are working
    //╭─────────┬───────────┬────────────┬───────────┬───────────╮╭────────┬────────┬────────┬────────┬─────────╮
      [K!("*|/"), K!("9"),    K!("8"),     K!("7"),    K!(","),     K!("'"), K!("!"), __,      K!("°"), K!("∑")],
    //├─────────┼───────────┼────────────┼───────────┼───────────┤├────────┼────────┼────────┼────────┼─────────┤
      [K!("+|-"), K!("3|l⌃"), K!("2|l⎇"), K!("1|l⌘"), K!("0|l⇧"),  K!("§"), K!("%"), K!("≤"), K!("≥"), K!("#")],
    //├─────────┼───────────┼────────────┼───────────┼───────────┤├────────┼────────┼────────┼────────┼─────────┤
      [K!("=|^"), K!("6"),    K!("5"),     K!("4"),    K!("."),     K!("_"), K!("µ"), K!("±"), K!("≈"), K!("≠")],
    //╰─────────┴───────────┴────────────╮                       ││                 ╭────────┴────────┴─────────╯
      [__,        __,         __,           __,        __,          __,      __,      __,      __,      __     ]
    //                                   ╰───────────┴───────────╯╰────────┴────────╯
  ]),
  layer!([//SYM
    // TODO HRM
    //╭─────────┬──────────┬──────────┬──────────┬─────────╮╭───────┬────────┬────────┬────────┬─────────╮
      [ K!("←"),  K!("↑"),   K!("↓"),   K!("→"),   K!("^"),  K!("`"), K!("?"), K!("▼"), K!("▼"), K!("▼")],
    //├─────────┼──────────┼──────────┼──────────┼─────────┤├───────┼────────┼────────┼────────┼─────────┤
      [K!("{|}"), K!("[|]"), K!("(|)"), K!("<|>"), K!("F⌫"), K!("!"), K!("@"), K!("="), K!("&"), K!("#")],
    //├─────────┼──────────┼──────────┼──────────┼─────────┤├───────┼────────┼────────┼────────┼─────────┤
      [__,        __,        K!("~"),   K!("$"),   K!("∖"),  K!("-"), K!("€"), __,      __,      __      ],
    //╰─────────┴──────────┴──────────╮                    ││                ╭────────┴────────┴─────────╯
      [__,        __,        __,        K!("W⌫"),  __,       __,      __,      __,      __,      __     ]
    //                                 ╰─────────┴─────────╯╰───────┴────────╯
  ]),
  layer!([//CMD
    // TODO think of Window mid
    // HRM with OSM on CRD layer?
    //╭─────────┬──────────┬────────┬──────────┬─────────╮╭──────────┬──────────┬─────────┬──────────┬───────────╮
      [K!("🔎∗"), K!("🔎"),  K!("↷"), K!("↶"),  K!("⇨⧉"),  K!("🔉"),  K!("🔇"),  K!("🔊"), K!("🔅"),  K!("🔆")], 
    //├─────────┼──────────┼────────┼──────────┼─────────┤├──────────┼──────────┼─────────┼──────────┼───────────┤
      [K!("✔*"), K!("✂"),    K!("⧉"), K!("📋"), K!("📋*"), K!("🖥️⬅"), K!("⬅▢"), __,       K!("▢🡺"), K!("🡺🖥️")],
    //├─────────┼──────────┼────────┼──────────┼─────────┤├──────────┼──────────┼─────────┼──────────┼───────────┤
      [K!("⇤🔍"), K!("🔎⇥"), K!("↩"), K!("⇥"),  K!("⇨▢"),   K!("⎋"),   K!("⏮"),  K!("⏯"),  K!("⏭"),  __],   
    //╰─────────┴──────────┴────────╮                    ││                     ╭─────────┴──────────┴───────────╯
      [__, __, __, __, __, __, __, __, __, __]
    //                              ╰──────────┴─────────╯╰──────────┴──────────╯
    ]),
  layer!([//CRD
    //╭─────┬─────┬─────┬─────┬─────╮╭─────┬─────┬─────┬────┬─────╮
    [k!(C), __, __, __, __, __, __, __, __, __],
    //├─────┼─────┼─────┼─────┼─────┤├─────┼─────┼─────┼────┼─────┤
    [osm!(LSFT), osm!(LCTL), osm!(LOPT), osm!(LCMD), osm!(LSFT), osm!(RSFT), osm!(RCMD), osm!(ROPT), osm!(RCTL), osm!(RSFT)],
    //├─────┼─────┼─────┼─────┼─────┤├─────┼─────┼─────┼────┼─────┤
    [__, __, __, __, __, __, __, __, __, __],
    //╰──────┴────────────┴────────────╮                         ││                             ╭────────────┴───────────┴───────╯
    [__, __, __, __, __, __, __, __, __, __]
    //                                 ╰────────────┴────────────╯╰────────────────┴────────────╯
  ]),
  layer!([//layer for VIAL modifications
    [__, __, __, __, __, __, __, __, __, __],
    [__, __, __, __, __, __, __, __, __, __],
    [__, __, __, __, __, __, __, __, __, __],
    [__, __, __, __, __, __, __, __, __, __]
  ]),
  ]
}

/// combos
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
            // = -> ^
            fork_alternative_shift(k!(KpEqual), wm!(I, LOPT)),
            // { -> }
            fork_alternative_shift(shifted!(LeftBracket), shifted!(RightBracket)),
            // ( -> )
            fork_alternative_shift(shifted!(Kc9), shifted!(Kc0)),
            // < -> >
            fork_alternative_shift(shifted!(Comma), shifted!(Dot)),
        ])
        .expect("Some fork is not valid"),
    }
}

/// modifiers
const LCMD: ModifierCombination = ModifierCombination::new_from(false, true, false, false, false);
const LOPT: ModifierCombination = ModifierCombination::new_from(false, false, true, false, false);
const LSFT: ModifierCombination = ModifierCombination::new_from(false, false, false, true, false);
const LCTL: ModifierCombination = ModifierCombination::new_from(false, false, false, false, true);
const RCMD: ModifierCombination = ModifierCombination::new_from(true, true, false, false, false);
const ROPT: ModifierCombination = ModifierCombination::new_from(true, false, true, false, false);
const RSFT: ModifierCombination = ModifierCombination::new_from(true, false, false, true, false);
const RCTL: ModifierCombination = ModifierCombination::new_from(true, false, false, false, true);

/// lt! but using layer names
const fn lt(layer: u8, key: KeyCode) -> KeyAction {
    KeyAction::LayerTapHold(Action::Key(key), layer)
}

/// layer names
const ALPHA: u8 = 0;
const NUM: u8 = 1;
const SYM: u8 = 2;
const CMD: u8 = 3;
const CRD: u8 = 4;

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
