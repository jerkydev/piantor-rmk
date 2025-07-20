use rmk::{a, k, layer, mo, mt, keycode::{ModifierCombination}, wm, action::KeyAction};
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
            [wm!(Kp1, ModifierCombination::SHIFT), wm!(Kp2, ModifierCombination::SHIFT), wm!(Kp3, ModifierCombination::SHIFT), wm!(Kp4, ModifierCombination::SHIFT), wm!(Kp5, ModifierCombination::SHIFT), wm!(Kp6, ModifierCombination::SHIFT), wm!(Kp7, ModifierCombination::SHIFT), wm!(Kp8, ModifierCombination::SHIFT), wm!(Kp9, ModifierCombination::SHIFT), wm!(Kp0, ModifierCombination::SHIFT)],
            [mt!(Kp1, ModifierCombination::CTRL), mt!(Kp2, ModifierCombination::ALT), mt!(Kp3, ModifierCombination::SHIFT), mt!(Kp4, ModifierCombination::GUI), k!(Kp5), k!(Kp6), mt!(Kp7, ModifierCombination::GUI), mt!(Kp8, ModifierCombination::SHIFT), mt!(Kp9, ModifierCombination::ALT), mt!(Kp0, ModifierCombination::CTRL)],
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