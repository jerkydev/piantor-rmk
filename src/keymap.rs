use rmk::{a, action::KeyAction, k, keycode::ModifierCombination, layer, mo, mt, wm};
pub(crate) const COL: usize = 10;
pub(crate) const ROW: usize = 4;
pub(crate) const NUM_LAYER: usize = 3;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [k!(Q), k!(W), k!(E), k!(R), k!(T), k!(Y), k!(U), k!(I), k!(O), k!(P)],
            [mt!(A, ModifierCombination::CTRL), mt!(S, ModifierCombination::ALT), mt!(D, ModifierCombination::SHIFT), mt!(F, ModifierCombination::GUI), k!(G), k!(H), mt!(J, ModifierCombination::GUI), mt!(K, ModifierCombination::SHIFT), mt!(L, ModifierCombination::ALT), mt!(Semicolon, ModifierCombination::CTRL)],
            [k!(Z), k!(X), k!(C), k!(V), k!(B), k!(N), k!(M), k!(Comma), k!(Dot), k!(Slash)],
            [k!(No), k!(No), k!(No), k!(Space), mo!(1), mo!(1), mo!(2), k!(No), k!(No), k!(No)]
        ]),
        layer!([
            [wm!(Kc1, ModifierCombination::SHIFT), wm!(Kc2, ModifierCombination::SHIFT), wm!(Kc3, ModifierCombination::SHIFT), wm!(Kc4, ModifierCombination::SHIFT), wm!(Kc5, ModifierCombination::SHIFT), wm!(Kc6, ModifierCombination::SHIFT), wm!(Kc7, ModifierCombination::SHIFT), wm!(Kc8, ModifierCombination::SHIFT), wm!(Kc9, ModifierCombination::SHIFT), wm!(Kc0, ModifierCombination::SHIFT)],
            [mt!(Kc1, ModifierCombination::CTRL), mt!(Kc2, ModifierCombination::ALT), mt!(Kc3, ModifierCombination::SHIFT), mt!(Kc4, ModifierCombination::GUI), k!(Kc5), k!(Kc6), mt!(Kc7, ModifierCombination::GUI), mt!(Kc8, ModifierCombination::SHIFT), mt!(Kc9, ModifierCombination::ALT), mt!(Kc0, ModifierCombination::CTRL)],
            [k!(F1), k!(F2), k!(F3), k!(F4), k!(F5), k!(F6), k!(F7), k!(F8), k!(F9), k!(F10)],
            [k!(No), k!(No), k!(No), k!(No), a!(Transparent), k!(F11), k!(F12), k!(No), k!(No), k!(No)]
        ]),
        layer!([
            [k!(MediaPrevTrack), k!(MediaPlayPause), k!(MediaNextTrack), k!(AudioVolDown), k!(AudioVolUp), k!(Escape), k!(CapsLock), k!(PageUp), k!(Backspace), k!(Delete)],
            [k!(LCtrl), k!(LAlt), k!(LShift), k!(LGui), k!(No), k!(Left), k!(Down), k!(Up), k!(Right), k!(Enter)],
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
