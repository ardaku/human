// human
//
// Copyright (c) 2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/// How the keyboard should be used if connected
#[derive(Copy, Clone, Debug)]
pub enum KeyboardMode {
    /// Keyboard controls focus
    Focus,
    /// Keyboard controls user actions
    Action,
    /// Keyboard controls typing
    Typing,
}

/// How the gamepad should be used if connected
#[derive(Copy, Clone, Debug)]
pub enum GamepadMode {
    /// Gamepad controls the GUI
    Focus,
    /// Gamepad controls user actions
    Action,
    /// Gamepad controls typing via on-screen keyboard
    Typing,
}

/// How the pointer should be used if connected
#[derive(Copy, Clone, Debug)]
pub enum PointerMode {
    /// Get where on the screen the pointer is
    Position,
    /// Get movement of the pointer
    Delta,
    /// Pointer controls typing via on-screen keyboard
    Typing,
}
