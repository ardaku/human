use pasts::{exec, wait};
use devout::{log, Tag};
use human::Input;

const INFO: Tag = Tag::new("Info").show(true);

/// The program's shared state.
struct State {}

/// Event handled by the event loop.
enum Event {
    Input(Input),
}

impl State {
    /// Event loop.
    fn event(&mut self, event: Event) {
        match event {
            Event::Input(input) => log!(INFO, "Input: {:?}", input),
        }
    }
}

/// Start the async executor.
fn main() {
    let mut state = State {};
    let mut input = human::Input::listener();

    exec!(state.event(wait! {
        Event::Input((&mut input).await),
    }));
}
