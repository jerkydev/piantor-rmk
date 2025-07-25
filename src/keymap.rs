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
pub(crate) const COL: usize = 10;
pub(crate) const ROW: usize = 4;
pub(crate) const NUM_LAYER: usize = 4;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [k!(Q), k!(W), k!(E), k!(R), k!(T), k!(Y), k!(U), k!(I), k!(O), k!(P)],
            [mt!(A, ModifierCombination::CTRL), mt!(S, ModifierCombination::ALT), mt!(D, ModifierCombination::SHIFT), mt!(F, ModifierCombination::GUI), k!(G), k!(H), mt!(J, ModifierCombination::GUI), mt!(K, ModifierCombination::SHIFT), mt!(L, ModifierCombination::ALT), mt!(Semicolon, ModifierCombination::CTRL)],
            [k!(Z), k!(X), k!(C), k!(V), k!(B), k!(N), k!(M), k!(Comma), k!(Dot), k!(NonusBackslash)],
            [k!(No), k!(No), k!(No), k!(Space), mo!(1), mo!(3), mo!(2), k!(No), k!(No), k!(No)]
        ]),
        layer!([
            [wm!(Kc1, ModifierCombination::SHIFT), wm!(Kc2, ModifierCombination::SHIFT), wm!(Kc3, ModifierCombination::SHIFT), wm!(Kc4, ModifierCombination::SHIFT), wm!(Kc5, ModifierCombination::SHIFT), wm!(Kc6, ModifierCombination::SHIFT), wm!(Kc7, ModifierCombination::SHIFT), wm!(Kc8, ModifierCombination::SHIFT), wm!(Kc9, ModifierCombination::SHIFT), wm!(Kc0, ModifierCombination::SHIFT)],
            [k!(Kc1), k!(Kc2), k!(Kc3), k!(Kc4), k!(Kc5), k!(Kc6), k!(Kc7), k!(Kc8), k!(Kc9), k!(Kc0)],
            [k!(F1), k!(F2), k!(F3), k!(F4), k!(F5), k!(F6), k!(F7), k!(F8), k!(F9), k!(F10)],
            [k!(No), k!(No), k!(No), k!(No), a!(Transparent), k!(F11), k!(F12), k!(No), k!(No), k!(No)]
        ]),
        layer!([
            [k!(MediaPrevTrack), k!(MediaPlayPause), k!(MediaNextTrack), k!(AudioVolDown), k!(AudioVolUp), k!(Escape), k!(CapsLock), k!(PageUp), k!(Backspace), k!(Delete)],
            [k!(No), k!(No), k!(No), k!(No), k!(No), k!(Left), k!(Down), k!(Up), k!(Right), k!(Enter)],
            [k!(No), k!(No), k!(No), k!(BrightnessDown), k!(BrightnessUp), k!(Home), k!(Tab), k!(PageDown), k!(End), k!(No)],
            [k!(No), k!(No), k!(Bootloader), k!(No), k!(No), k!(No), a!(Transparent), k!(No), k!(No), k!(No)]
        ]),
        layer!([
            [wm!(Q, ModifierCombination::ALT), wm!(Equal, ModifierCombination::SHIFT), wm!(Kc3, ModifierCombination::SHIFT), wm!(Kc4, ModifierCombination::SHIFT), wm!(Kc7, ModifierCombination::SHIFT), wm!(Slash, ModifierCombination::SHIFT), wm!(U, ModifierCombination::ALT), wm!(I, ModifierCombination::ALT), wm!(O, ModifierCombination::ALT), wm!(P, ModifierCombination::ALT)],
            [wm!(Minus, ModifierCombination::SHIFT), k!(Equal), wm!(D, ModifierCombination::ALT), wm!(F, ModifierCombination::ALT), wm!(Kc1, ModifierCombination::SHIFT), wm!(H, ModifierCombination::ALT), wm!(Kc0, ModifierCombination::SHIFT), wm!(Kc9, ModifierCombination::SHIFT), wm!(L, ModifierCombination::ALT), wm!(Semicolon, ModifierCombination::ALT)],
            [wm!(Z, ModifierCombination::ALT), k!(Minus), wm!(C, ModifierCombination::ALT), wm!(V, ModifierCombination::ALT), wm!(Kc8, ModifierCombination::SHIFT), wm!(N, ModifierCombination::ALT), wm!(Kc2, ModifierCombination::SHIFT), wm!(Comma, ModifierCombination::ALT), wm!(Dot, ModifierCombination::ALT), wm!(Slash, ModifierCombination::ALT)],
            [k!(No), k!(No), k!(No), wm!(Kc5, ModifierCombination::SHIFT), wm!(Kc6, ModifierCombination::SHIFT), k!(No), k!(No), k!(No), k!(No), k!(No)]
        ]),
        // layer!([
        //     [k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No)],
        //     [k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No)],
        //     [k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No)],
        //     [k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No), k!(No)]
        // ]),
    ]
}

pub fn get_combo_config() -> CombosConfig {
    CombosConfig {
        timeout: Duration::from_millis(50),
        combos: [
            // -
            Combo::new([k!(W), k!(E)], k!(Minus), Some(0)),
            // =
            Combo::new([k!(E), k!(R)], k!(Equal), Some(0)),
            // LMEH
            Combo::new(
                [k!(C), k!(V)],
                KeyAction::Single(Action::Modifier(
                    ModifierCombination::SHIFT
                        | ModifierCombination::ALT
                        | ModifierCombination::CTRL,
                )),
                Some(0),
            ),
            // RMEH
            Combo::new(
                [k!(M), k!(Comma)],
                KeyAction::Single(Action::Modifier(
                    ModifierCombination::SHIFT
                        | ModifierCombination::ALT
                        | ModifierCombination::CTRL,
                )),
                Some(0),
            ),
            // LHYPER
            Combo::new(
                [k!(X), k!(C)],
                KeyAction::Single(Action::Modifier(
                    ModifierCombination::SHIFT
                        | ModifierCombination::ALT
                        | ModifierCombination::CTRL
                        | ModifierCombination::GUI,
                )),
                Some(0),
            ),
            // RHYPER
            Combo::new(
                [k!(Comma), k!(Dot)],
                KeyAction::Single(Action::Modifier(
                    ModifierCombination::SHIFT
                        | ModifierCombination::ALT
                        | ModifierCombination::CTRL
                        | ModifierCombination::GUI,
                )),
                Some(0),
            ),
        ]
        .into_iter()
        .collect(),
    }
}
