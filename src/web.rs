//! Since the web (javascript) input isn't connected to creating a window by
//! talking to a window manager, input can be received here.

use std::{task::{Waker, Poll, Context}};

use web_sys::{EventTarget, InputEvent, KeyboardEvent, HtmlElement, HtmlInputElement};
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

static mut KEYBOARD_STATE: u128 = 0;

#[allow(unsafe_code)]
fn key_up_state(key: Key) -> bool {
    let bitflag = 1 << (key as i8);
    unsafe {
        let ret = KEYBOARD_STATE & bitflag != 0;
        KEYBOARD_STATE &= !bitflag;
        ret
    }
}

#[allow(unsafe_code)]
fn key_down_state(key: Key) -> bool {
    let bitflag = 1 << (key as i8);
    unsafe {
        let ret = !(KEYBOARD_STATE & bitflag != 0);
        KEYBOARD_STATE |= bitflag;
        ret
    }
}

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
        "ControlLeft" | "OSLeft" => Key::LCtrl,
        "ControlRight" | "OSRight" => Key::RCtrl,
        "ShiftLeft" => Key::LShift,
        "ShiftRight" => Key::RShift,
        "Space" => Key::Space,
        "Tab" => Key::Tab,
        "Backspace" => Key::Delete,
        "Delete" => Key::Del,
        "Escape" => Key::Back,
        "NumpadEnter" | "Enter" => Key::Enter,
        "CapsLock" => Key::CapsLk,
        "Minus" => Key::Minus,
        "Equal" => Key::Equal,
        "Insert" => Key::Lang,
        "PageUp" => Key::PgUp,
        "PageDown" => Key::PgDn,
        "Home" => Key::Home,
        "End" => Key::End,
        "Backslash" => Key::Backslash,
        "BracketRight" => Key::BracketClose,
        "BracketLeft" => Key::BracketOpen,
        "Quote" => Key::Apostrophe,
        "Semicolon" => Key::Semicolon,
        "Slash" => Key::Slash,
        "Period" | "NumpadDecimal" => Key::Period,
        "Comma" => Key::Comma,
        "Pause" => Key::Pause,
        "ScrollLock" => Key::ScrollLk,
        "ContextMenu" => Key::Menu,
        "Backquote" => Key::Backtick,
        "F1" => Key::F1,
        "F2" => Key::F2,
        "F3" => Key::F3,
        "F4" => Key::F4,
        "F5" => Key::F5,
        "F6" => Key::F6,
        "F7" => Key::F7,
        "F8" => Key::F8,
        "F9" => Key::F9,
        "F10" => Key::F10,
        "F11" => Key::F11,
        "F12" => Key::F12,
        "NumLock" => Key::ScrollLk,
        "NumpadDivide" => Key::Slash,
        "NumpadMultiply" => Key::Eight,
        "NumpadSubtract" => Key::Minus,
        "NumpadAdd" => Key::Equal,
        x => return None,
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
    let localized_input = localized_input.into();
    web_sys::window().unwrap().document().unwrap()
        .get_elements_by_tag_name("body")
        .get_with_index(0)
        .unwrap()
        .append_child(&localized_input)
        .unwrap();
    let localized_input: EventTarget = localized_input.into();
    let focus: Closure<dyn Fn()> =
        Closure::wrap(Box::new(move || {
            let localized_input: HtmlElement = web_sys::window().unwrap().document().unwrap()
                .get_element_by_id("rust_crate_human__").unwrap().dyn_into().unwrap();
            localized_input.focus().unwrap();
        }));
    let focus = focus.into_js_value();
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
    blur.forget();

    let input: Closure<dyn Fn(InputEvent)> =
        Closure::wrap(Box::new(move |event: InputEvent| {
            if !event.is_composing() {
                let localized_input = web_sys::window().unwrap().document().unwrap()
                    .get_element_by_id("rust_crate_human__").unwrap();
                let localized_input: HtmlInputElement = localized_input.dyn_into().unwrap();
                let value = localized_input.value();
                localized_input.set_value("");
                let mut char_iter = value.chars();
                'a: while let Some(ref waker) = state().waker {
                    if let Some(c) = char_iter.next() {
                        state().input = Some(Input::Text(c));
                        waker.wake_by_ref();
                    } else {
                        break 'a;
                    }
                }
            }
            event.stop_propagation();
        }));
    localized_input.add_event_listener_with_callback(
        "input",
        &input.as_ref().unchecked_ref()
    ).unwrap();
    input.forget();

    #[allow(trivial_casts)] // Actually needed here.
    let key_down: Closure<dyn Fn(KeyboardEvent)> =
        Closure::wrap(Box::new(move |event: KeyboardEvent| {
            // If a input is being `.await`ed, wake the waiting thread.
            if let Some(ref waker) = state().waker {
                // Set future to complete.
                if let Some(keycode) = keycode(&event.code()) {
                    if key_down_state(keycode) {
                        state().input = Some(Input::KeyPress(keycode));
                    } else {
                        state().input = None;
                    }
                } else {
                    state().input = None;
                }
                // Wake the microphone future.
                waker.wake_by_ref();
            }
            event.stop_propagation();
            // FIXME: Prevent default on non-printing keys
        }));
    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback(
            "keydown",
            key_down.as_ref().unchecked_ref(),
        )
        .unwrap();
    key_down.forget();

    #[allow(trivial_casts)] // Actually needed here.
    let key_up: Closure<dyn Fn(KeyboardEvent)> =
        Closure::wrap(Box::new(move |event: KeyboardEvent| {
            // If a input is being `.await`ed, wake the waiting thread.
            if let Some(ref waker) = state().waker {
                // Set future to complete.
                if let Some(keycode) = keycode(&event.code()) {
                    if key_up_state(keycode) {
                        state().input = Some(Input::KeyRelease(keycode));
                    } else {
                        state().input = None;
                    }
                } else {
                    state().input = None;
                }
                // Wake the microphone future.
                waker.wake_by_ref();
            }
            event.stop_propagation();
            // FIXME: Prevent default on non-printing keys
        }));
    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback(
            "keyup",
            key_up.as_ref().unchecked_ref(),
        )
        .unwrap();
    key_up.forget();
    
    let localized_input: HtmlElement = localized_input.dyn_into().unwrap();
    localized_input.focus().unwrap();
}
