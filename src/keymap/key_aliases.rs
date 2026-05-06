use rmk::a;
use rmk::types::action::KeyAction;

/// key shortcuts
macro_rules! K {
    ("␣") => {
        k!(Space)
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
        lt(
            NUM,
            rmk::types::keycode::KeyCode::Hid(rmk::types::keycode::HidKeyCode::Backspace),
        )
    };
    ("🔁|🅛SYM") => {
        lt(
            SYM,
            rmk::types::keycode::KeyCode::Hid(rmk::types::keycode::HidKeyCode::Again),
            // rmk::types::action::KeyAction::Single(rmk::types::action::Action::Special(
            //     rmk::types::keycode::SpecialKey::Repeat,
            // )),
        )
    };
    ("␣|🅛⇉") => {
        lt(
            CRD,
            rmk::types::keycode::KeyCode::Hid(rmk::types::keycode::HidKeyCode::Space),
        )
    };
    ("⏎|🅛⌘") => {
        lt(
            CMD,
            rmk::types::keycode::KeyCode::Hid(rmk::types::keycode::HidKeyCode::Enter),
        )
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
        rmk::types::action::KeyAction::Single(rmk::types::action::Action::Special(
            rmk::types::keycode::SpecialKey::GraveEscape,
        ))
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
pub(crate) const XX: KeyAction = a!(No);
pub(crate) const __: KeyAction = a!(Transparent);
