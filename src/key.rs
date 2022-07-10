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
    #[inline(always)]
    pub fn none(self) -> bool {
        self.0 == 0
    }

    /// Check if SHIFT is held down.
    #[inline(always)]
    pub fn shift(self) -> bool {
        (self.0 & MOD_SHIFT) != 0
    }

    /// Check if CTRL or CMD is held down.
    #[inline(always)]
    pub fn ctrl(self) -> bool {
        (self.0 & MOD_CTRL) != 0
    }

    /// Check if ALT or OPTION is held down.
    #[inline(always)]
    pub fn alt(self) -> bool {
        (self.0 & MOD_ALT) != 0
    }

    /// No modifiers.
    #[inline(always)]
    pub fn new() -> Self {
        Mod(0)
    }

    /// Add shift key.
    #[inline(always)]
    pub fn add_shift(self) -> Self {
        Self(self.0 | MOD_SHIFT)
    }

    /// Add control key.
    #[inline(always)]
    pub fn add_ctrl(self) -> Self {
        Self(self.0 | MOD_CTRL)
    }

    /// Add alt key.
    #[inline(always)]
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
    /// The Escape Key
    Esc = 0x00u8,
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
    Backslash = 0x0D,
    /// Backtick / Tilde Key
    Backtick = 0x0E,
    /// Tab
    Tab = 0x0F,
    /// Q (may be named by a different glyph depending on language of user).
    Q = 0x10,
    /// W (may be named by a different glyph depending on language of user).
    W = 0x11,
    /// E (may be named by a different glyph depending on language of user).
    E = 0x12,
    /// R (may be named by a different glyph depending on language of user).
    R = 0x13,
    /// T (may be named by a different glyph depending on language of user).
    T = 0x14,
    /// Y (may be named by a different glyph depending on language of user).
    Y = 0x15,
    /// U (may be named by a different glyph depending on language of user).
    U = 0x16,
    /// I (may be named by a different glyph depending on language of user).
    I = 0x17,
    /// U (may be named by a different glyph depending on language of user).
    O = 0x18,
    /// I (may be named by a different glyph depending on language of user).
    P = 0x19,
    /// [ (may be named by a different glyph depending on language of user).
    BracketOpen = 0x1A,
    /// ] (may be named by a different glyph depending on language of user).
    BracketClose = 0x1B,
    /// Backspace / Delete key.
    Delete = 0x1C,
    /// Key Caps or Caps Lock Key
    Caps = 0x1D,
    /// A (may be named by a different glyph depending on language of user).
    A = 0x1E,
    /// S (may be named by a different glyph depending on language of user).
    S = 0x1F,
    /// D (may be named by a different glyph depending on language of user).
    D = 0x20,
    /// F (may be named by a different glyph depending on language of user).
    F = 0x21,
    /// G (may be named by a different glyph depending on language of user).
    G = 0x22,
    /// H (may be named by a different glyph depending on language of user).
    H = 0x23,
    /// J (may be named by a different glyph depending on language of user).
    J = 0x24,
    /// K (may be named by a different glyph depending on language of user).
    K = 0x25,
    /// L (may be named by a different glyph depending on language of user).
    L = 0x26,
    /// ; (may be named by a different glyph depending on language of user).
    Semicolon = 0x27,
    /// ' (may be named by a different glyph depending on language of user).
    Apostrophe = 0x28,
    /// Return / Enter Key
    Enter = 0x29,
    /// Left Shift Key
    LShift = 0x2A,
    /// Z (may be named by a different glyph depending on language of user).
    Z = 0x2B,
    /// X (may be named by a different glyph depending on language of user).
    X = 0x2C,
    /// C (may be named by a different glyph depending on language of user).
    C = 0x2D,
    /// V (may be named by a different glyph depending on language of user).
    V = 0x2E,
    /// B (may be named by a different glyph depending on language of user).
    B = 0x2F,
    /// N (may be named by a different glyph depending on language of user).
    N = 0x30,
    /// M (may be named by a different glyph depending on language of user).
    M = 0x31,
    /// , (may be named by a different glyph depending on language of user).
    Comma = 0x32,
    /// . (may be named by a different glyph depending on language of user).
    Period = 0x33,
    /// / (may be named by a different glyph depending on language of user).
    Slash = 0x34,
    /// Right Shift Key
    RShift = 0x35,
    /// Up Arrow
    Up = 0x36,
    /// Left control
    LCtrl = 0x37,
    /// Left alt
    LAlt = 0x38,
    /// Space (or Left Thumb Button)
    Space = 0x39,
    /// Thumb Key (Alt Gr or Right Thumb Button)
    Thumb = 0x3A,
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
    
    /// The Power Key
    Power = 0x40,
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
    /// Print Screen / Screenshot Key
    Screenshot = 0x4D,
    /// Insert Key
    Insert = 0x4E,
    /// Next Media Track Key
    Next = 0x4F,
    /// Toggle Microphone
    Microphone = 0x50,
    /// Web Launcher (WWW Key)
    Web = 0x51,
    /// E-mail Launcher (Mail Key)
    Email = 0x52,
    // R = 0x53,
    // T = 0x54,
    // Y = 0x55,
    // U = 0x56,
    // I = 0x57,
    // O = 0x58,
    /// Audio Player Launcher
    Player = 0x59,
    // BracketOpen = 0x5A,
    /// Calculator Launcher
    Calculator = 0x5B,
    /// The Del/Delete Key
    Del = 0x5C,
    /// Previous Media Track Key
    Prev = 0x5D,
    /// Airplane Mode
    Airplane = 0x5E,
    /// Scroll Lock
    Scroll = 0x5F,
    /// Display Toggle
    Display = 0x60,
    // F = 0x61,
    // G = 0x62,
    /// Korean Han Key
    Han = 0x63,
    /// Korean Hanja Key
    Hanja = 0x64,
    /// Kana Toggle Key
    Kana = 0x65,
    // L = 0x66,
    /// Convert Key
    Convert = 0x67,
    /// Non-Convert Key
    NonConvert = 0x68,
    /// Numpad Enter Key
    NumEnter = 0x69,
    /// Mute Key
    Mute = 0x6B,
    /// Volume Down Key
    Quieter = 0x6C,
    /// Volume Up Key
    Louder = 0x6D,
    /// Brightness Down Key
    Dimmer = 0x6E,
    /// Brightness Up Key
    Brighter = 0x6F,
    /// Num Lock
    Num = 0x70,
    /// Context Menu
    Menu = 0x71,
    /// Comma
    Break = 0x72,
    /// Period
    Clear = 0x73,
    /// Pause
    Pause = 0x74,
    /// Page Up
    PageUp = 0x76,
    /// Multimedia Track Play Key
    Play = 0x79,
    /// Multimedia Track Stop Key
    Stop = 0x7A,
    /// Home
    Home = 0x7D,
    /// Page Down
    PageDown = 0x7E,
    /// End
    End = 0x7F,

    // RESERVED: LShift = 0x6A,
    // RESERVED: RShift = 0x75,
    // RESERVED: LCtrl = 0x77,
    // RESERVED: LAlt = 0x78,
    // RESERVED: RAlt = 0x7B,
    // RESERVED: RCtrl = 0x7C
}
