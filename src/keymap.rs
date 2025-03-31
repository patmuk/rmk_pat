use rmk::action::KeyAction;
// use rmk::{a, k, layer, mo};
use rmk::{a, k, layer};
pub(crate) const COL: usize = 10;
pub(crate) const ROW: usize = 4;
pub(crate) const NUM_LAYER: usize = 2;
#[rustfmt::skip]
pub fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [k!(Kc0),   k!(Kc1),    k!(Kc2),    k!(Kc3),    k!(Kc4),    k!(Kc5),    k!(Kc6),    k!(Kc7),    k!(Kc8),    k!(Kc9)],
            [k!(A),     k!(B),      k!(C),      k!(D),      k!(E),      k!(F),      k!(G),      k!(H),      k!(I),      k!(J)],
            [k!(K),     k!(L),      k!(M),      k!(N),      k!(O),      k!(P),      k!(Q),      k!(R),      k!(S),      k!(T)],            
            [a!(No),    a!(No),     a!(No),     k!(W),      k!(X),      k!(Y),      k!(Z),      a!(No),     a!(No),     a!(No)]
        ]),
        layer!([
            [k!(Grave), k!(F1), k!(F2), k!(F3), k!(F4), k!(F5), k!(F6), k!(F7), k!(F8), k!(Delete)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [k!(CapsLock), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), k!(UP)]
        ]),
    ]
}
