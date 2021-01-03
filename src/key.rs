// Human
// Copyright © 2020-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use std::fmt::{Debug, Formatter, Result};

const MOD_SHIFT: u8 = 0b0000_0001;
const MOD_CTRL: u8 = 0b0000_0010;
const MOD_ALT: u8 = 0b0000_0100;
const MOD_FUNC: u8 = 0b0000_1000;

const MOD_CTRL_SHIFT: u8 = MOD_CTRL | MOD_SHIFT;
const MOD_ALT_SHIFT: u8 = MOD_ALT | MOD_SHIFT;
const MOD_CTRL_ALT: u8 = MOD_CTRL | MOD_ALT;
const MOD_CTRL_ALT_SHIFT: u8 = MOD_CTRL_SHIFT | MOD_ALT_SHIFT;

/// Modifier state.
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct Mod(u8);

impl Mod {
    /// Check if no modifiers are held down.
    pub fn none(self) -> bool {
        self.0 == 0
    }

    /// Check if SHIFT is held down.
    pub fn shift(self) -> bool {
        (self.0 & MOD_SHIFT) == MOD_SHIFT
    }

    /// Check if CTRL or CMD is held down.
    ///
    /// Also triggered with SHIFT+ALT
    pub fn ctrl(self) -> bool {
        let is_ctrl = (self.0 & MOD_CTRL_ALT_SHIFT) == MOD_CTRL;
        let is_shift_alt = (self.0 & MOD_CTRL_ALT_SHIFT) == MOD_ALT_SHIFT;

        is_ctrl || is_shift_alt
    }

    /// Check if ALT or OPTION is held down.
    ///
    /// Also triggered with SHIFT+CTRL
    pub fn alt(self) -> bool {
        let is_shift_ctrl = (self.0 & MOD_CTRL_ALT_SHIFT) == MOD_CTRL_SHIFT;
        let is_alt = (self.0 & MOD_CTRL_ALT_SHIFT) == MOD_ALT;

        is_shift_ctrl || is_alt
    }

    /// Check if FN is held down.
    ///
    /// Also triggered with CTRL+ALT
    pub fn func(self) -> bool {
        let is_func = (self.0 & MOD_FUNC) == MOD_FUNC;
        let is_ctrl_alt = (self.0 & MOD_CTRL_ALT) == MOD_CTRL_ALT;

        is_func || is_ctrl_alt
    }

    /// No modifiers.
    pub fn new() -> Self {
        Mod(0)
    }

    /// Add shift key.
    pub fn add_shift(self) -> Self {
        Self(self.0 | MOD_SHIFT)
    }

    /// Add control key.
    pub fn add_ctrl(self) -> Self {
        Self(self.0 | MOD_CTRL)
    }

    /// Add alt key.
    pub fn add_alt(self) -> Self {
        Self(self.0 | MOD_ALT)
    }

    /// Add fn key.
    pub fn add_fn(self) -> Self {
        Self(self.0 | MOD_FUNC)
    }
}

impl Debug for Mod {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.ctrl() {
            write!(f, "Ctrl")?;
        } else if self.alt() {
            write!(f, "Alt")?;
        } else if self.shift() {
            write!(f, "Shift")?;
            if self.func() {
                write!(f, " + Fn")?;
            }
        } else if self.func() {
            write!(f, "Fn")?;
        } else {
            write!(f, "None")?;
        }
        Ok(())
    }
}

/// Input keycode for a key on a keyboard.  Each key is assigned a number from
/// 0 to 63 (mapped to a minimal 64-key keyboard).
///
/// # The 64-Key Keyboard
/// ![The 64-Key Keyboard Picture](https://raw.githubusercontent.com/libcala/human/main/res/keyboard.png)
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Key {
    /// Also known as the ESCAPE key.
    Back = 0x00u8,
    /// Numeric 1 on either top row or numpad.
    One = 0x01,
    /// Numeric 2 on either top row or numpad.
    Two = 0x02,
    /// Numeric 3 on either top row or numpad.
    Three = 0x03,
    /// Numeric 4 on either top row or numpad.
    Four = 0x04,
    /// Numeric 5 on either top row or numpad.
    Five = 0x05,
    /// Numeric 6 on either top row or numpad.
    Six = 0x06,
    /// Numeric 7 on either top row or numpad.
    Seven = 0x07,
    /// Numeric 8 on either top row or numpad.
    Eight = 0x08,
    /// Numeric 9 on either top row or numpad.
    Nine = 0x09,
    /// Numeric 0 on either top row or numpad.
    Zero = 0x0A,
    /// Minus / Underscore Key
    Minus = 0x0B,
    /// Equal Sign / Plus Key
    Equal = 0x0C,
    /// Backtick / Tilde Key
    Backtick = 0x0D,
    /// Tab
    Tab = 0x0E,
    /// Q (may be named by a different glyph depending on language of user).
    Q = 0x0F,
    /// W (may be named by a different glyph depending on language of user).
    W = 0x10,
    /// E (may be named by a different glyph depending on language of user).
    E = 0x11,
    /// R (may be named by a different glyph depending on language of user).
    R = 0x12,
    /// T (may be named by a different glyph depending on language of user).
    T = 0x13,
    /// Y (may be named by a different glyph depending on language of user).
    Y = 0x14,
    /// U (may be named by a different glyph depending on language of user).
    U = 0x15,
    /// I (may be named by a different glyph depending on language of user).
    I = 0x16,
    /// U (may be named by a different glyph depending on language of user).
    O = 0x17,
    /// I (may be named by a different glyph depending on language of user).
    P = 0x18,
    /// [ (may be named by a different glyph depending on language of user).
    BracketOpen = 0x19,
    /// ] (may be named by a different glyph depending on language of user).
    BracketClose = 0x1A,
    /// Delete (Also Backspace).
    Delete = 0x1B,
    /// Env (Also known as: Win, Super, Cmd, Search) Key
    Env = 0x1C,
    /// A (may be named by a different glyph depending on language of user).
    A = 0x1D,
    /// S (may be named by a different glyph depending on language of user).
    S = 0x1E,
    /// D (may be named by a different glyph depending on language of user).
    D = 0x1F,
    /// F (may be named by a different glyph depending on language of user).
    F = 0x20,
    /// G (may be named by a different glyph depending on language of user).
    G = 0x21,
    /// H (may be named by a different glyph depending on language of user).
    H = 0x22,
    /// J (may be named by a different glyph depending on language of user).
    J = 0x23,
    /// K (may be named by a different glyph depending on language of user).
    K = 0x24,
    /// L (may be named by a different glyph depending on language of user).
    L = 0x25,
    /// ; (may be named by a different glyph depending on language of user).
    Semicolon = 0x26,
    /// ' (may be named by a different glyph depending on language of user).
    Apostrophe = 0x27,
    /// Enter (Also Return).
    Enter = 0x28,
    /// Left Shift Key
    LShift = 0x29,
    /// Z (may be named by a different glyph depending on language of user).
    Z = 0x2A,
    /// X (may be named by a different glyph depending on language of user).
    X = 0x2B,
    /// C (may be named by a different glyph depending on language of user).
    C = 0x2C,
    /// V (may be named by a different glyph depending on language of user).
    V = 0x2D,
    /// B (may be named by a different glyph depending on language of user).
    B = 0x2E,
    /// N (may be named by a different glyph depending on language of user).
    N = 0x2F,
    /// M (may be named by a different glyph depending on language of user).
    M = 0x30,
    /// , (may be named by a different glyph depending on language of user).
    Comma = 0x31,
    /// . (may be named by a different glyph depending on language of user).
    Period = 0x32,
    /// / (may be named by a different glyph depending on language of user).
    Slash = 0x33,
    /// \ (may be named by a different glyph depending on language of user).
    Backslash = 0x34,
    /// Up Arrow
    Up = 0x35,
    /// Right Shift Key
    RShift = 0x36,
    /// Left control
    LCtrl = 0x37,
    /// Left alt
    LAlt = 0x38,
    /// Space (or Left Thumb Button)
    Space = 0x39,
    /// Compose Key (Alt Gr, Right Thumb Button, or Lock Key)
    Compose = 0x3A,
    /// Right Alt
    RAlt = 0x3B,
    /// Right Control
    RCtrl = 0x3C,
    /// Left Arrow Key
    Left = 0x3D,
    /// Down Arrow Key
    Down = 0x3E,
    /// Right Arrow Key
    Right = 0x3F,
}
