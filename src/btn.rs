// Human
// Copyright © 2020-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

/// Input keycode for a button on a mouse.
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Btn {
    /// Left or Primary click
    Left = 0x00u8,
    /// Middle click or Scroll click
    Middle = 0x01u8,
    /// Right Click
    Right = 0x02u8,
    /// Back Button
    Back = 0x03u8,
    /// Next Button
    Next = 0x04u8,
    /// DPI Button
    Dpi = 0x05u8,
    /// Extra Mouse Button
    Extra = 0x06u8,
}
