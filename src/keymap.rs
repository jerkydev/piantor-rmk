use embassy_time::Duration;
use rmk::{
    a,
    action::{Action, KeyAction},
    combo::Combo,
    config::CombosConfig,
    k,
    keycode::ModifierCombination,
    layer, mo, wm,
};
pub(crate) const COL: usize = 10;
pub(crate) const ROW: usize = 4;
pub(crate) const NUM_LAYER: usize = 3;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [k!(Q), k!(W), k!(E), k!(R), k!(T), k!(Y), k!(U), k!(I), k!(O), k!(P)],
            [k!(A), k!(S), k!(D), k!(F), k!(G), k!(H), k!(J), k!(K), k!(L), k!(Semicolon)],
            [k!(Z), k!(X), k!(C), k!(V), k!(B), k!(N), k!(M), k!(Comma), k!(Dot), k!(Slash)],
            [k!(No), k!(No), k!(No), k!(Space), mo!(1), mo!(1), mo!(2), k!(No), k!(No), k!(No)]
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
                [k!(B), k!(G)],
                KeyAction::Single(Action::Modifier(
                    ModifierCombination::SHIFT
                        | ModifierCombination::ALT
                        | ModifierCombination::CTRL,
                )),
                Some(0),
            ),
            // RMEH
            Combo::new(
                [k!(N), k!(H)],
                KeyAction::Single(Action::Modifier(
                    ModifierCombination::SHIFT
                        | ModifierCombination::ALT
                        | ModifierCombination::CTRL,
                )),
                Some(0),
            ),
            // LHYPER
            Combo::new(
                [k!(T), k!(G)],
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
                [k!(Y), k!(H)],
                KeyAction::Single(Action::Modifier(
                    ModifierCombination::SHIFT
                        | ModifierCombination::ALT
                        | ModifierCombination::CTRL
                        | ModifierCombination::GUI,
                )),
                Some(0),
            ),
            // LCTRL
            Combo::new(
                [k!(A), k!(Z)],
                KeyAction::Single(Action::Modifier(ModifierCombination::CTRL)),
                Some(0),
            ),
            // LALT
            Combo::new(
                [k!(S), k!(X)],
                KeyAction::Single(Action::Modifier(ModifierCombination::ALT)),
                Some(0),
            ),
            // LSHIFT
            Combo::new(
                [k!(D), k!(C)],
                KeyAction::Single(Action::Modifier(ModifierCombination::SHIFT)),
                Some(0),
            ),
            // LCMD
            Combo::new(
                [k!(F), k!(V)],
                KeyAction::Single(Action::Modifier(ModifierCombination::GUI)),
                Some(0),
            ),
            // RCTRL
            Combo::new(
                [k!(Semicolon), k!(Slash)],
                KeyAction::Single(Action::Modifier(ModifierCombination::CTRL)),
                Some(0),
            ),
            // RALT
            Combo::new(
                [k!(L), k!(Dot)],
                KeyAction::Single(Action::Modifier(ModifierCombination::ALT)),
                Some(0),
            ),
            // RSHIFT
            Combo::new(
                [k!(K), k!(Comma)],
                KeyAction::Single(Action::Modifier(ModifierCombination::SHIFT)),
                Some(0),
            ),
            // RCMD
            Combo::new(
                [k!(J), k!(M)],
                KeyAction::Single(Action::Modifier(ModifierCombination::GUI)),
                Some(0),
            ),
        ]
        .into_iter()
        .collect(),
    }
}
