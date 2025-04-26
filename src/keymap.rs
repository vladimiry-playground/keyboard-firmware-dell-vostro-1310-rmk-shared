use rmk::action::KeyAction;
use rmk::{a, k, layer, mo};

pub(crate) const COL: usize = 16;
pub(crate) const ROW: usize = 8;
pub(crate) const NUM_LAYER: usize = 2;

// https://docs.rs/rmk/latest/rmk/index.html#macros

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
        [
        layer!([
            [  k!(Escape),  k!(F3),  k!(F5),  k!(F6),  k!(F7),    k!(F9),       k!(F10),         k!(F11),         k!(Home),    k!(Delete),    mo!(1),     a!(No),    a!(No),   a!(No),   a!(No),   a!(No)],
            [      k!(F1),  k!(F2),  k!(F4), k!(Kc6), k!(Kc8),    k!(F8),       k!(Kc9),         k!(F12),          k!(End),    k!(Insert),    a!(No), k!(LShift),    a!(No),   a!(No),   a!(No),   a!(No)],
            [   k!(Grave), k!(Kc2), k!(Kc4), k!(Kc5), k!(Kc7),     k!(I),         k!(O),         k!(Kc0),        k!(Equal), k!(Backspace),    a!(No),     a!(No), k!(LCtrl),   a!(No),   a!(No),   a!(No)],
            [     k!(Kc1),   k!(Q), k!(Kc3),   k!(T),   k!(U),     k!(J),         k!(P),       k!(Minus), k!(RightBracket),     k!(Right),    a!(No),     a!(No), k!(RCtrl),   a!(No),   a!(No),   a!(No)],
            [     k!(Tab),   k!(W),   k!(E),   k!(R),   k!(Y),     k!(K),         k!(L), k!(LeftBracket),    k!(Backslash),     k!(Enter),    a!(No),     a!(No),    a!(No), k!(LGui),   a!(No),   a!(No)],
            [k!(CapsLock),   k!(S),   k!(D),   k!(G),   k!(H),     k!(M), k!(Semicolon),       k!(Quote),           k!(Up),        a!(No),    a!(No), k!(RShift),    a!(No),   a!(No),   a!(No),   a!(No)],
            [       k!(A),   k!(X),   k!(C),   k!(F),   k!(N), k!(Comma),       k!(Dot),       k!(Slash),       k!(PageUp),  k!(PageDown),    a!(No),     a!(No),    a!(No),   a!(No), k!(LAlt),   a!(No)],
            [      a!(No),   k!(Z),  a!(No),   k!(V),   k!(B), k!(Space),        a!(No),          a!(No),         k!(Left),      k!(Down),    a!(No),     a!(No),    a!(No),   a!(No), k!(RAlt), k!(Menu)]
        ]),
        layer!([
            [  k!(Escape),  k!(F3),  k!(F5),  k!(F6),  k!(F7),    k!(F9),       k!(F10), k!(PrintScreen),         k!(Home),      k!(AudioMute), mo!(1),     a!(No),    a!(No),   a!(No),   a!(No),  a!(No)],
            [      k!(F1),  k!(F2),  k!(F4), k!(Kc6), k!(Kc8),    k!(F8),       k!(Kc9),         k!(F12),          k!(End),         k!(Insert), a!(No), k!(LShift),    a!(No),   a!(No),   a!(No),   a!(No)],
            [   k!(Grave), k!(Kc2), k!(Kc4), k!(Kc5), k!(Kc7),     k!(I),         k!(O),         k!(Kc0),        k!(Equal),      k!(Backspace), a!(No),     a!(No), k!(LCtrl),   a!(No),   a!(No),   a!(No)],
            [     k!(Kc1),   k!(Q), k!(Kc3),   k!(T),   k!(U),     k!(J),         k!(P),       k!(Minus), k!(RightBracket),     k!(AudioVolUp), a!(No),     a!(No), k!(RCtrl),   a!(No),   a!(No),   a!(No)],
            [     k!(Tab),   k!(W),   k!(E),   k!(R),   k!(Y),     k!(K),         k!(L), k!(LeftBracket),    k!(Backslash),          k!(Enter), a!(No),     a!(No),    a!(No), k!(LGui),   a!(No),   a!(No)],
            [k!(CapsLock),   k!(S),   k!(D),   k!(G),   k!(H),     k!(M), k!(Semicolon),       k!(Quote), k!(BrightnessUp),             a!(No), a!(No), k!(RShift),    a!(No),   a!(No),   a!(No),   a!(No)],
            [       k!(A),   k!(X),   k!(C),   k!(F),   k!(N), k!(Comma),       k!(Dot),       k!(Slash),   k!(AudioVolUp),   k!(AudioVolDown), a!(No),     a!(No),    a!(No),   a!(No), k!(LAlt),   a!(No)],
            [      a!(No),   k!(Z),  a!(No),   k!(V),   k!(B), k!(Space),        a!(No),          a!(No), k!(AudioVolDown), k!(BrightnessDown), a!(No),     a!(No),    a!(No),   a!(No), k!(RAlt), k!(Menu)]
        ]),
    ]
}
