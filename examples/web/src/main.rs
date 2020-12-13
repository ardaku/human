use std::cell::RefCell;

use pasts::prelude::*;
use devout::{log, Tag};

const INFO: Tag = Tag::new("Info").show(true);

/// The program's shared state.
struct State {}

/// Program start.
async fn start() {
    // Initialize shared state.
    let _state = RefCell::new(State {});
    // Get Human User Input Listener
    let mut listener = human::Input::listener();

    loop {
        log!(INFO, "Input: {:?}", (&mut listener).await);
    }
}

/// Start the async executor.
fn main() {
    exec!(start());
}
