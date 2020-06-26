// human
//
// Copyright (c) 2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/// How the keyboard/gamepad/pointer should be used if connected
#[derive(Copy, Clone, Debug)]
pub enum Mode {
    /// Device controls (usually graphical) user interface
    Ui,
    /// Device controls game actions
    Game,
    /// Device controls text input
    Text,
}
