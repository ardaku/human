// Human
// Copyright © 2020-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::{Controls, Key, Mod, Btn};

/// A gamepad, flightstick, smartphone, or other controller.
#[derive(Debug)]
pub struct Controller(Box<stick::Controller>);

impl Controller {
    /// Get a unique identifier for the specific model of this controller.
    pub fn id(&self) -> [u16; 4] {
        self.0.id()
    }

    /// The name of the controller.
    pub fn name(&self) -> String {
        self.0.name()
    }

    /// Turn on/off haptic force feedback. Set `power` between 0.0 (off) and 1.0
    /// (maximum vibration). Anything outside that range will be clamped.
    pub fn rumble(&mut self, power: f32) {
        self.0.rumble(power)
    }
}

impl Future for Controller {
    type Output = Controls;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.get_mut().0).poll(cx)
    }
}

/// Input event from any human interface device
#[derive(Debug)]
#[non_exhaustive]
pub enum Input {
    /// User inputted text.
    Text(char),
    /// A key on the keyboard was pressed or released.
    Key(Mod, Key, bool),
    /// Pointer button was pressed or released (Left click, Tap).
    Click(Mod, Btn, bool),
    /// The pointer has left the window.
    PointerLeave,
    /// Request to shift the viewport in the X dimension (relative coordinates).
    ScrollX(Mod, f32),
    /// Request to shift the viewport in the Y dimension (relative coordinates).
    ScrollY(Mod, f32),
    /// The pointer was moved in the X dimension (absolute coordinates).
    PointerX(f32),
    /// The pointer was moved in the Y dimension (absolute coordinates).
    PointerY(f32),
    /// Touchscreen was touched with one finger.
    Touch(bool),
    /// Touchscreen was touched with two fingers.
    Pinch(bool),
    /// Pinch width has changed.
    PinchW(f32),
    /// Pinch height has changed.
    PinchH(f32),
    /// Rotation Amount
    PinchZ(f32),
    /// New controller plugged in.
    Controller(Controller),
}

struct InputListener<T: Future<Output = (usize, Controls)> + Unpin> {
    ctlr: T,
}

impl<T> Future for InputListener<T>
where
    T: Future<Output = (usize, Controls)> + Unpin,
{
    type Output = Input;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if let Poll::Ready((_, Controls::Connect(new))) =
            Pin::new(&mut this.ctlr).poll(cx)
        {
            return Poll::Ready(Input::Controller(Controller(new)));
        }

        #[cfg(target_arch = "wasm32")]
        {
            crate::web::poll(cx)
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            Poll::Pending
        }
    }
}

impl Input {
    /// Get a future that returns input events.  You may only call this once,
    /// because multiple threads reading the same input wouldn't logically work.
    pub fn listener() -> impl Future<Output = Input> {
        #[cfg(target_arch = "wasm32")]
        crate::web::init();

        InputListener {
            ctlr: stick::Controller::listener(),
        }
    }
}
