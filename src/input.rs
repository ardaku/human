use std::{pin::Pin, task::{Context, Poll}, future::Future};

use crate::{Key, Controls};

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
pub enum Input {
    /// User inputted text.
    Text(char),
    /// A key on the keyboard was pressed.
    KeyPress(Key),
    /// A key on the keyboard was released.
    KeyRelease(Key),
    /// The pointer was moved in the X dimension (absolute coordinates).
    PointerX(f64),
    /// The pointer was moved in the Y dimension (absolute coordinates).
    PointerY(f64),
    /// The pointer has left the window.
    PointerLeave,
    /// Request to shift the viewport in the X dimension (relative coordinates).
    ScrollX(f64),
    /// Request to shift the viewport in the Y dimension (relative coordinates).
    ScrollY(f64),
    /// 2-Finger Pinch Starting Width Changed.
    PinchW(f64),
    /// 2-Finger Pinch Starting Height Changed.
    PinchH(f64),
    /// 2-Finger Pinch Width Change.
    ScaleW(f64),
    /// 2-Finger Pinch Height Change.
    ScaleH(f64),
    /// New controller plugged in.
    Controller(Controller),
}

struct InputListener<T: Future<Output = (usize, Controls)> + Unpin> {
    ctlr: T
}

impl<T> Future for InputListener<T>
    where T: Future<Output = (usize, Controls)> + Unpin
{
    type Output = Input;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if let Poll::Ready((_, Controls::Connect(new))) = Pin::new(&mut this.ctlr).poll(cx) {
            return Poll::Ready(Input::Controller(Controller(new)));
        }
        #[cfg(target_arch = "wasm32")] { crate::web::poll(cx) }
        #[cfg(not(target_arch = "wasm32"))] { Poll::Pending }
    }
}

impl Input {
    /// Get a future that returns input events.  You may only call this once,
    /// because multiple threads reading the same input wouldn't logically work.
    pub fn listener() -> impl Future<Output = Input> {
        #[cfg(target_arch = "wasm32")]
        crate::web::init();

        InputListener {
            ctlr: stick::Controller::listener()
        }
    }
}
