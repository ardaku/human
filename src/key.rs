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
    pub fn ctrl(self) -> bool {
        (self.0 & MOD_CTRL) == MOD_CTRL
    }

    /// Check if ALT or OPTION is held down.
    pub fn alt(self) -> bool {
        (self.0 & MOD_ALT) == MOD_ALT
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
}

impl Debug for Mod {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.ctrl() {
            if self.alt() {
                if self.shift() {
                    write!(f, "Ctrl + Alt + Shift")
                } else {
                    write!(f, "Ctrl + Alt")
                }
            } else {
                write!(f, "Ctrl")
            }
        } else if self.alt() {
            if self.shift() {
                write!(f, "Alt + Shift")
            } else {
                write!(f, "Alt")
            }
        } else if self.shift() {
            write!(f, "Shift")
        } else {
            write!(f, "None")
        }
    }
}

/// Input keycode for a key on a keyboard.
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
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
    /// Backspace.
    Backspace = 0x1B,
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
    /// Compose Key (Alt Gr, Right Thumb Button, NumLock, ScrLk Key)
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
    
    // Back = 0x40u8,
    /// F1 Key
    F1 = 0x41,
    /// F2 Key
    F2 = 0x42,
    /// F3 Key
    F3 = 0x43,
    /// F4 Key
    F4 = 0x44,
    /// F5 Key
    F5 = 0x45,
    /// F6 Key
    F6 = 0x46,
    /// F7 Key
    F7 = 0x47,
    /// F8 Key
    F8 = 0x48,
    /// F9 Key
    F9 = 0x49,
    /// F10 Key
    F10 = 0x4A,
    /// F11 Key
    F11 = 0x4B,
    /// F12 Key
    F12 = 0x4C,
    /// Insert Key
    Insert = 0x4D,
    // Tab = 0x4E,
    // Q = 0x4F,
    // W = 0x50,
    // E = 0x51,
    // R = 0x52,
    // T = 0x53,
    // Y = 0x54,
    // U = 0x55,
    // I = 0x56,
    // O = 0x57,
    // P = 0x58,
    // BracketOpen = 0x19,
    // BracketClose = 0x1A,
    /// The delete key
    Delete = 0x5B,
    /// The Caps Lock Key
    CapsLock = 0x5C,
    // A = 0x1D,
    // S = 0x1E,
    // D = 0x1F,
    // F = 0x20,
    // G = 0x21,
    // H = 0x22,
    // J = 0x23,
    // K = 0x24,
    // L = 0x25,
    // Semicolon = 0x26,
    // Apostrophe = 0x27,
    // NumpadEnter = 0x28,
    // LShift = 0x29,
    // Z = 0x2A,
    // X = 0x2B,
    // C = 0x2C,
    // V = 0x2D,
    // B = 0x2E,
    // N = 0x2F,
    // M = 0x30,
    // Comma = 0x31,
    // Period = 0x32,
    // Slash = 0x33,
    // Backslash = 0x34,
    /// Page Up
    PageUp = 0x75,
    // RShift = 0x36,
    // LCtrl = 0x37,
    // LAlt = 0x38,
    /// Pause Key
    Pause = 0x79,
    // Compose = 0x3A,
    // RAlt = 0x3B,
    /// Context Menu
    Menu = 0x7C,
    /// Home
    Home = 0x7D,
    /// Page Down
    PageDown = 0x7E,
    /// End
    End = 0x7F,
}
