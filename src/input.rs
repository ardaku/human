use std::{pin::Pin, task::{Context, Poll}, future::Future};

use crate::Key;

// It's defenitely reachable, bug in rustc.
#[allow(unreachable_pub)]
pub use stick::{Controller, Event as Controls};

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
    Controller(Box<Controller>),
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
            return Poll::Ready(Input::Controller(new));
        }
        // FIXME: Check web keyboard input as well.
        Poll::Pending
    }
}

impl Input {
    /// Get a future that returns input events.
    pub fn listener() -> impl Future<Output = Input> {
        InputListener {
            ctlr: Controller::listener()
        }
    }
}
