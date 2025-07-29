use embassy_time::Duration;
use rmk::{
    a,
    action::{Action, KeyAction},
    combo::Combo,
    config::CombosConfig,
    k,
    keycode::ModifierCombination,
    layer, mo, mt, wm,
};
const COL: usize = 10;
const ROW: usize = 4;
const NUM_LAYER: usize = 5;

const ALT: ModifierCombination = ModifierCombination::ALT;
const SHIFT: ModifierCombination = ModifierCombination::SHIFT;
const GUI: ModifierCombination = ModifierCombination::GUI;
const CTRL: ModifierCombination = ModifierCombination::CTRL;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [k!(Q),        k!(W),       k!(E),         k!(R),       k!(T),          k!(Y),  k!(U),       k!(I),         k!(O),       k!(P)],
            [mt!(A, CTRL), mt!(S, ALT), mt!(D, SHIFT), mt!(F, GUI), k!(G),          k!(H),  mt!(J, GUI), mt!(K, SHIFT), mt!(L, ALT), mt!(Semicolon, CTRL)],
            [k!(Z),        k!(X),       k!(C),         k!(V),       k!(B),          k!(N),  k!(M),       k!(Comma),     k!(Dot),     k!(NonusBackslash)],
            [k!(No),       k!(No),      k!(No),        k!(Space),   mo!(1),         mo!(2), mo!(3),      k!(No),        k!(No),      k!(No)]
        ]),
        layer!([
            [wm!(Q, ALT),       wm!(Equal, SHIFT), wm!(Kc3, SHIFT), wm!(Kc4, SHIFT),  k!(No),                   k!(Equal),        wm!(Kc0, SHIFT), wm!(Kc9, SHIFT), k!(No),        k!(No)],
            [k!(Kc1),           k!(Kc2),           k!(Kc3),         k!(Kc4),          k!(Kc5),                  k!(Kc6),          k!(Kc7),         k!(Kc8),         k!(Kc9),       k!(Kc0)],
            [wm!(Minus, SHIFT), k!(Minus),         k!(No),          k!(No),           wm!(Kc8, SHIFT),          wm!(Kc5, SHIFT),  wm!(Kc6, SHIFT), wm!(Comma, ALT), wm!(Dot, ALT), k!(Slash)],
            [k!(No),            k!(No),            k!(No),          mo!(4),           a!(Transparent),          a!(No),           k!(No),          k!(No),          k!(No),        k!(No)]
        ]),
        layer!([
            [wm!(Q, ALT),       wm!(Equal, SHIFT), wm!(Kc3, SHIFT),         wm!(Kc4, SHIFT),          wm!(Kc7, SHIFT),          k!(Equal),         wm!(Kc0, SHIFT),  wm!(Kc9, SHIFT), wm!(O, ALT),   wm!(P, ALT)],
            [wm!(A, ALT),       wm!(Slash, SHIFT), wm!(LeftBracket, SHIFT), wm!(RightBracket, SHIFT), wm!(Kc1, SHIFT),          wm!(H, ALT),       k!(RightBracket), k!(LeftBracket), wm!(L, ALT),   wm!(Semicolon, ALT)],
            [wm!(Minus, SHIFT), k!(Minus),         wm!(Comma, SHIFT),       wm!(Dot, SHIFT),          wm!(Kc8, SHIFT),          wm!(Kc5, SHIFT),   wm!(Kc6, SHIFT),  wm!(Comma, ALT), wm!(Dot, ALT), k!(Slash)],
            [k!(No),            k!(No),            k!(No),                  k!(Backslash),            wm!(Kc2, SHIFT),          a!(Transparent),   k!(No),           k!(No),          k!(No),        k!(No)]
        ]),
        layer!([
            [k!(MediaPrevTrack), k!(MediaPlayPause), k!(MediaNextTrack), k!(AudioVolDown), k!(AudioVolUp),          k!(Escape), k!(Backspace),   k!(PageUp),   k!(Delete),    k!(BrightnessUp)],
            [k!(F1),             k!(F2),             k!(F3),             k!(F4),           k!(F5),                  k!(Left),   k!(Down),        k!(Up),       k!(Right),     k!(Enter)],
            [k!(F6),             k!(F7),             k!(F8),             k!(F9),           k!(F10),                 k!(Home),   k!(Tab),         k!(PageDown), k!(End),       k!(BrightnessDown)],
            [k!(No),             k!(No),             k!(No),             k!(F11),          k!(F12),                 k!(No),     a!(Transparent), k!(No),       k!(No),        k!(No)]
        ]),
        layer!([
            [k!(Bootloader), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No)],
            [k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No)],
            [k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No)],
            [k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No)]
        ]),
    ]
}

pub fn get_combo_config() -> CombosConfig {
    let meh = KeyAction::Single(Action::Modifier(SHIFT | ALT | CTRL));
    let hyper = KeyAction::Single(Action::Modifier(SHIFT | ALT | CTRL | GUI));
    CombosConfig {
        timeout: Duration::from_millis(50),
        combos: [
            // -
            Combo::new([k!(W), k!(E)], k!(Minus), Some(0)),
            // =
            Combo::new([k!(E), k!(R)], k!(Equal), Some(0)),
            // CapsLock
            Combo::new([k!(U), k!(I)], k!(CapsLock), Some(0)),
            // LMEH
            Combo::new([k!(C), k!(V)], meh, Some(0)),
            // RMEH
            Combo::new([k!(M), k!(Comma)], meh, Some(0)),
            // LHYPER
            Combo::new([k!(X), k!(C)], hyper, Some(0)),
            // RHYPER
            Combo::new([k!(Comma), k!(Dot)], hyper, Some(0)),
        ]
        .into_iter()
        .collect(),
    }
}
