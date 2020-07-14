// human
//
// Copyright (c) 2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::sync::atomic::{AtomicU8, Ordering};

pub(super) static KEYBOARD: AtomicU8 = AtomicU8::new(Mode::Text as u8);
pub(super) static GAMEPAD: AtomicU8 = AtomicU8::new(Mode::Game as u8);
pub(super) static POINTER: AtomicU8 = AtomicU8::new(Mode::Ui as u8);

/// How the keyboard/gamepad/pointer should be used if connected
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Mode {
    /// Device controls (usually graphical) user interface
    Ui = 0u8,
    /// Device controls game actions
    Game = 1u8,
    /// Device controls text input
    Text = 2u8,
}

/// Set the input mode for the different types of input devices.
pub fn set_mode(keyboard: Mode, gamepad: Mode, pointer: Mode) {
    KEYBOARD.store(keyboard as u8, Ordering::Relaxed);
    GAMEPAD.store(gamepad as u8, Ordering::Relaxed);
    POINTER.store(pointer as u8, Ordering::Relaxed);
}

fn mode(value: u8) -> Mode {
    match value {
        0 => Mode::Ui,
        1 => Mode::Game,
        2 => Mode::Text,
        _ => unreachable!(),
    }
}

/// Get the current input mode of the keyboard.
pub fn mode_keyboard() -> Mode {
    mode(KEYBOARD.load(Ordering::Relaxed))
}

/// Get the current input mode of the gamepad.
pub fn mode_gamepad() -> Mode {
    mode(GAMEPAD.load(Ordering::Relaxed))
}

/// Get the current input mode of the pointer.
pub fn mode_pointer() -> Mode {
    mode(POINTER.load(Ordering::Relaxed))
}
