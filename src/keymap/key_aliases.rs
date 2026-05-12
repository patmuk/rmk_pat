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
    ("⌫") => {
        k!(Backspace)
    };
    ("W⌫") => {
        wm!(Backspace, ROPT)
    };
    ("⌫|🅛NUM") => {
        lt(NUM, rmk::types::keycode::KeyCode::Backspace)
    };
    ("🔁|🅛SYM") => {
        // Note: rmk intercepts `KeyCode::Again` locally and substitutes it
        // with `last_key_code` (see rmk::keyboard::process_action). It does
        // NOT actually transmit the HID Again keycode to the host.
        lt(SYM, rmk::types::keycode::KeyCode::Again)
    };
    ("␣|🅛⇉") => {
        lt(CRD, rmk::types::keycode::KeyCode::Space)
    };
    ("⏎|🅛⌘") => {
        lt(CMD, rmk::types::keycode::KeyCode::Enter)
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
    ("+|-|l⇧") => {
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
        k!(Grave)
    };
    ("?") => {
        shifted!(Slash)
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
        wm!(F, LCMD)
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
    // focus prev window (Amethyst focus-ccw)
    ("⬅▢") => {
        wm!(Left, RCTL.bitor(ROPT))
    };
    // focus next window (Amethyst focus-cw)
    ("▢🡺") => {
        wm!(Right, RCTL.bitor(ROPT))
    };
    ("🡺🖥️") => {
        wm!(Right, RCTL)
    };
    // cycle layout (Amethyst cycle-layout)
    ("▦↻") => {
        wm!(Space, RCTL.bitor(ROPT))
    };
    // close window / tab
    ("✕▢") => {
        wm!(W, LCMD)
    };
    // quit app
    ("⏻") => {
        wm!(Q, LCMD)
    };
    // minimize window
    ("🗕") => {
        wm!(M, LCMD)
    };
    // findPrev
    ("⇤🔍") => {
        wm!(G, LCMD.bitor(LSFT))
    };
    //fNext Enter  tab   switchWin   ESC   prev  play/pause next
    ("🔎⇥") => {
        wm!(G, LCMD)
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
    // ===== HRM on SYM layer =====
    // SYM home row, left: { / [ / ( / < / F⌫  with l⇧ ⌃ ⎇ ⌘ ⇧ holds (mirrors alpha)
    ("{|}|l⇧") => {
        wmt(rmk::types::keycode::KeyCode::LeftBracket, LSFT, LSFT)
    };
    ("[|]|l⌃") => {
        mt!(LeftBracket, LCTL)
    };
    ("(|)|l⎇") => {
        wmt(rmk::types::keycode::KeyCode::Kc9, LSFT, LOPT)
    };
    ("<|>|l⌘") => {
        wmt(rmk::types::keycode::KeyCode::Comma, LSFT, LCMD)
    };
    ("F⌫|l⇧") => {
        wmt(rmk::types::keycode::KeyCode::D, LCTL, LSFT)
    };
    // SYM home row, right: ! / @ / = / & / #  with r⇧ ⌘ ⎇ ⌃ ⇧ holds
    ("!|r⇧") => {
        wmt(rmk::types::keycode::KeyCode::Kc1, LSFT, RSFT)
    };
    ("@|r⌘") => {
        wmt(rmk::types::keycode::KeyCode::Kc2, LSFT, RCMD)
    };
    ("=|r⎇") => {
        mt!(Equal, ROPT)
    };
    ("&|r⌃") => {
        wmt(rmk::types::keycode::KeyCode::Kc7, LSFT, RCTL)
    };
    ("#|r⇧") => {
        wmt(rmk::types::keycode::KeyCode::Kc3, LSFT, RSFT)
    };
    // ===== HRM on NUM layer (right side; left side already had HRM) =====
    ("§|r⇧") => {
        wmt(rmk::types::keycode::KeyCode::Kc6, ROPT, RSFT)
    };
    ("%|r⌘") => {
        wmt(rmk::types::keycode::KeyCode::Kc5, LSFT, RCMD)
    };
    ("≤|r⎇") => {
        wmt(rmk::types::keycode::KeyCode::Comma, ROPT, ROPT)
    };
    ("≥|r⌃") => {
        wmt(rmk::types::keycode::KeyCode::Dot, ROPT, RCTL)
    };
    // ===== HRM on CMD layer =====
    // CMD home row, left
    ("✔*|l⇧") => {
        wmt(rmk::types::keycode::KeyCode::A, LCMD, LSFT)
    };
    ("✂|l⌃") => {
        wmt(rmk::types::keycode::KeyCode::X, LCMD, LCTL)
    };
    ("⧉|l⎇") => {
        wmt(rmk::types::keycode::KeyCode::C, LCMD, LOPT)
    };
    ("📋|l⌘") => {
        wmt(rmk::types::keycode::KeyCode::V, LCMD, LCMD)
    };
    ("📋*|l⇧") => {
        wmt(rmk::types::keycode::KeyCode::V, LCMD.bitor(LSFT), LSFT)
    };
    // CMD home row, right (col 7 stays transparent → no HRM)
    ("🖥️⬅|r⇧") => {
        wmt(rmk::types::keycode::KeyCode::Left, RCTL, RSFT)
    };
    ("⬅▢|r⌘") => {
        wmt(rmk::types::keycode::KeyCode::Left, RCTL.bitor(ROPT), RCMD)
    };
    ("▢🡺|r⌃") => {
        wmt(rmk::types::keycode::KeyCode::Right, RCTL.bitor(ROPT), RCTL)
    };
    ("🡺🖥️|r⇧") => {
        wmt(rmk::types::keycode::KeyCode::Right, RCTL, RSFT)
    };
    // Catch-all case to trigger compile-time error if the string doesn't match
    ($key:expr) => {
        compile_error!(concat!("Unmatched key: ", $key))
    };
}
pub(crate) const XX: KeyAction = a!(No);
pub(crate) const __: KeyAction = a!(Transparent);
