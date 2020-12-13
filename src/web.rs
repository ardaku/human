//! Since the web (javascript) input isn't connected to creating a window by
//! talking to a window manager, input can be received here.

use std::{task::{Waker, Poll, Context}};

use web_sys::{EventTarget, InputEvent, KeyboardEvent, HtmlElement, Attr};
use wasm_bindgen::{closure::Closure, JsCast};

use crate::{Key, Input};

struct WebInput {
    input: Option<Input>,
    waker: Option<Waker>,
}

static mut WEB_INPUT: WebInput = WebInput {
    input: None,
    waker: None,
};

#[allow(unsafe_code)]
fn state<'a>() -> &'a mut WebInput {
    unsafe {
        &mut WEB_INPUT
    }
}

pub(crate) fn poll(cx: &mut Context<'_>) -> Poll<Input> {
    let state = state();
    if let Some(input) = state.input.take() {
        Poll::Ready(input)
    } else {
        state.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

/// Coverter for js keycode to human keycode.
fn keycode(keycode: &str) -> Option<Key> {
    let key = match keycode {
        "Digit0" | "Numpad0" => Key::Zero,
        "Digit1" | "Numpad1" => Key::One,
        "Digit2" | "Numpad2" => Key::Two,
        "Digit3" | "Numpad3" => Key::Three,
        "Digit4" | "Numpad4" => Key::Four,
        "Digit5" | "Numpad5" => Key::Five,
        "Digit6" | "Numpad6" => Key::Six,
        "Digit7" | "Numpad7" => Key::Seven,
        "Digit8" | "Numpad8" => Key::Eight,
        "Digit9" | "Numpad9" => Key::Nine,
        "KeyA" => Key::A,
        "KeyB" => Key::B,
        "KeyC" => Key::C,
        "KeyD" => Key::D,
        "KeyE" => Key::E,
        "KeyF" => Key::F,
        "KeyG" => Key::G,
        "KeyH" => Key::H,
        "KeyI" => Key::I,
        "KeyJ" => Key::J,
        "KeyK" => Key::K,
        "KeyL" => Key::L,
        "KeyM" => Key::M,
        "KeyN" => Key::N,
        "KeyO" => Key::O,
        "KeyP" => Key::P,
        "KeyQ" => Key::Q,
        "KeyR" => Key::R,
        "KeyS" => Key::S,
        "KeyT" => Key::T,
        "KeyU" => Key::U,
        "KeyV" => Key::V,
        "KeyW" => Key::W,
        "KeyX" => Key::X,
        "KeyY" => Key::Y,
        "KeyZ" => Key::Z,
        "ArrowUp" => Key::Up,
        "ArrowDown" => Key::Down,
        "ArrowLeft" => Key::Left,
        "ArrowRight" => Key::Right,
        "AltLeft" => Key::LAlt,
        "AltRight" => Key::RAlt,
        "ControlLeft" => Key::LCtrl,
        "ControlRight" => Key::RCtrl,
        "ShiftLeft" => Key::LShift,
        "ShiftRight" => Key::RShift,
        "Space" => Key::Space,
        "Tab" => Key::Tab,
        "Backspace" => Key::Delete,
        "Escape" => Key::Back,
        "NumpadEnter" | "Enter" => Key::Enter,
        "CapsLock" => Key::CapsLk,
        _ => return None,
    };
    Some(key)
}

/// One type future initialization for key presses and mouse events.
pub(crate) fn init() {
    let localized_input = web_sys::window().unwrap().document().unwrap()
        .create_element("input").unwrap();
    localized_input.set_attribute("style", "\
        border:0;\
        padding:0;\
        margin:0;\
        position:fixed;\
        top:0;\
        left:0;\
        width:0;\
        height:0;\
    ").unwrap();
    localized_input.set_attribute("id", "rust_crate_human__").unwrap();
    let window_node = web_sys::window().unwrap().document().unwrap();
    let localized_input = localized_input.into();
    window_node.append_child(&localized_input).unwrap();
    let localized_input: EventTarget = localized_input.into();
    let focus: Closure<dyn Fn()> =
        Closure::wrap(Box::new(move || {
            let localized_input: HtmlElement = web_sys::window().unwrap().document().unwrap()
                .get_element_by_id("rust_crate_human__").unwrap().dyn_into().unwrap();
            localized_input.focus().unwrap();
        }));
    let blur: Closure<dyn Fn()> =
        Closure::wrap(Box::new(move || {
            web_sys::window().unwrap().set_timeout_with_callback(
                &focus.as_ref().unchecked_ref()
            ).unwrap();
        }));
    localized_input.add_event_listener_with_callback(
        "blur",
        &blur.as_ref().unchecked_ref()
    ).unwrap();
    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback(
            "focus",
            &blur.as_ref().unchecked_ref(),
        )
        .unwrap();

    let input: Closure<dyn Fn(InputEvent)> =
        Closure::wrap(Box::new(move |event: InputEvent| {
            if !event.is_composing() {
                let localized_input = web_sys::window().unwrap().document().unwrap()
                    .get_element_by_id("rust_crate_human__").unwrap();
                let localized_input: Attr = localized_input.dyn_into().unwrap();
                let value = localized_input.value();
                let mut char_iter = value.chars();
                'a: while let Some(waker) = state().waker.take() {
                    if let Some(c) = char_iter.next() {
                        // Set future to complete.
                        state().input = Some(Input::Text(c));
                        // Wake the microphone future.
                        waker.wake();
                    } else {
                        break 'a;
                    }
                }
            }
        }));
    localized_input.add_event_listener_with_callback(
        "input",
        &input.as_ref().unchecked_ref()
    ).unwrap();

    #[allow(trivial_casts)] // Actually needed here.
    let key_down: Closure<dyn Fn(KeyboardEvent)> =
        Closure::wrap(Box::new(move |event: KeyboardEvent| {
            // If a input is being `.await`ed, wake the waiting thread.
            if let Some(waker) = state().waker.take() {
                // Set future to complete.
                if let Some(keycode) = keycode(&event.code()) {
                    state().input = Some(Input::KeyPress(keycode));
                } else {
                    state().input = None;
                }
                // Wake the microphone future.
                waker.wake();
            }
        }));
    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback(
            "keydown",
            key_down.as_ref().unchecked_ref(),
        )
        .unwrap();

    #[allow(trivial_casts)] // Actually needed here.
    let key_up: Closure<dyn Fn(KeyboardEvent)> =
        Closure::wrap(Box::new(move |event: KeyboardEvent| {
            // If a input is being `.await`ed, wake the waiting thread.
            if let Some(waker) = state().waker.take() {
                // Set future to complete.
                if let Some(keycode) = keycode(&event.code()) {
                    state().input = Some(Input::KeyRelease(keycode));
                } else {
                    state().input = None;
                }
                // Wake the microphone future.
                waker.wake();
            }
        }));
    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback(
            "keyup",
            key_up.as_ref().unchecked_ref(),
        )
        .unwrap();
}
