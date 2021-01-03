// Human
// Copyright © 2020-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

//! Since the web (javascript) input isn't connected to creating a window by
//! talking to a window manager, input can be received here.

use std::{
    sync::atomic::{AtomicU64, Ordering::SeqCst},
    task::{Context, Poll, Waker},
};

use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{
    EventTarget, HtmlElement, HtmlInputElement, InputEvent, KeyboardEvent,
};

use crate::{Input, Key, Mod};

struct WebInput {
    input: Option<Input>,
    waker: Option<Waker>,
}

static mut WEB_INPUT: WebInput = WebInput {
    input: None,
    waker: None,
};

static KEYBOARD_STATE: AtomicU64 = AtomicU64::new(0);

fn key_up_state(key: Key) -> bool {
    let bitflag = 1 << (key as i8);
    let new = KEYBOARD_STATE.fetch_and(!bitflag, SeqCst);
    let old = KEYBOARD_STATE.load(SeqCst);

    old != new
}

fn key_down_state(key: Key) -> bool {
    let bitflag = 1 << (key as i8);
    let new = KEYBOARD_STATE.fetch_or(bitflag, SeqCst);
    let old = KEYBOARD_STATE.load(SeqCst);

    old != new
}

#[allow(unsafe_code)]
fn state<'a>() -> &'a mut WebInput {
    unsafe { &mut WEB_INPUT }
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

fn is_printing(keycode: &str) -> bool {
    keycode.starts_with("Digit")
        || keycode.starts_with("Numpad")
        || keycode.starts_with("Key")
        || matches!(
            keycode,
            "Space"
                | "Minus"
                | "Equal"
                | "Backslash"
                | "BracketRight"
                | "BracketLeft"
                | "Quote"
                | "Semicolon"
                | "Slash"
                | "Period"
                | "Comma"
                | "Backquote"
        )
}

fn sys_modifier(event: &KeyboardEvent, mods: &mut Mod) -> bool {
    let ctrl =
        event.get_modifier_state("Control") || event.get_modifier_state("Meta");
    let alt = event.get_modifier_state("Alt");
    let shift = event.get_modifier_state("Shift");

    if ctrl {
        *mods = mods.add_ctrl();
    }

    if alt {
        *mods = mods.add_alt();
    }

    if shift {
        *mods = mods.add_shift();
    }

    ctrl || alt
}

/// Coverter for js keycode to human keycodes.
fn keycode(keycode: &str) -> Option<(Key, Mod)> {
    let key = match keycode {
        "Digit0" | "Numpad0" => (Key::Zero, Mod::new()),
        "Digit1" | "Numpad1" => (Key::One, Mod::new()),
        "Digit2" | "Numpad2" => (Key::Two, Mod::new()),
        "Digit3" | "Numpad3" => (Key::Three, Mod::new()),
        "Digit4" | "Numpad4" => (Key::Four, Mod::new()),
        "Digit5" | "Numpad5" => (Key::Five, Mod::new()),
        "Digit6" | "Numpad6" => (Key::Six, Mod::new()),
        "Digit7" | "Numpad7" => (Key::Seven, Mod::new()),
        "Digit8" | "Numpad8" => (Key::Eight, Mod::new()),
        "Digit9" | "Numpad9" => (Key::Nine, Mod::new()),
        "KeyA" => (Key::A, Mod::new()),
        "KeyB" => (Key::B, Mod::new()),
        "KeyC" => (Key::C, Mod::new()),
        "KeyD" => (Key::D, Mod::new()),
        "KeyE" => (Key::E, Mod::new()),
        "KeyF" => (Key::F, Mod::new()),
        "KeyG" => (Key::G, Mod::new()),
        "KeyH" => (Key::H, Mod::new()),
        "KeyI" => (Key::I, Mod::new()),
        "KeyJ" => (Key::J, Mod::new()),
        "KeyK" => (Key::K, Mod::new()),
        "KeyL" => (Key::L, Mod::new()),
        "KeyM" => (Key::M, Mod::new()),
        "KeyN" => (Key::N, Mod::new()),
        "KeyO" => (Key::O, Mod::new()),
        "KeyP" => (Key::P, Mod::new()),
        "KeyQ" => (Key::Q, Mod::new()),
        "KeyR" => (Key::R, Mod::new()),
        "KeyS" => (Key::S, Mod::new()),
        "KeyT" => (Key::T, Mod::new()),
        "KeyU" => (Key::U, Mod::new()),
        "KeyV" => (Key::V, Mod::new()),
        "KeyW" => (Key::W, Mod::new()),
        "KeyX" => (Key::X, Mod::new()),
        "KeyY" => (Key::Y, Mod::new()),
        "KeyZ" => (Key::Z, Mod::new()),
        "ArrowUp" => (Key::Up, Mod::new()),
        "ArrowDown" => (Key::Down, Mod::new()),
        "ArrowLeft" => (Key::Left, Mod::new()),
        "ArrowRight" => (Key::Right, Mod::new()),
        "Space" => (Key::Space, Mod::new()),
        "Tab" => (Key::Tab, Mod::new()),
        "Backspace" => (Key::Delete, Mod::new()),
        "Delete" => (Key::Delete, Mod::new().add_fn()),
        "Escape" => (Key::Back, Mod::new()),
        "NumpadEnter" | "Enter" => (Key::Enter, Mod::new()),
        "Minus" => (Key::Minus, Mod::new()),
        "Equal" => (Key::Equal, Mod::new()),
        "Insert" => (Key::Enter, Mod::new().add_fn()),
        "PageUp" => (Key::Up, Mod::new().add_fn()),
        "PageDown" => (Key::Down, Mod::new().add_fn()),
        "Home" => (Key::Left, Mod::new().add_fn()),
        "End" => (Key::Right, Mod::new().add_fn()),
        "Backslash" => (Key::Backslash, Mod::new()),
        "BracketLeft" => (Key::BracketOpen, Mod::new()),
        "BracketRight" => (Key::BracketClose, Mod::new()),
        "Quote" => (Key::Apostrophe, Mod::new()),
        "Semicolon" => (Key::Semicolon, Mod::new()),
        "Slash" => (Key::Slash, Mod::new()),
        "Period" | "NumpadDecimal" => (Key::Period, Mod::new()),
        "Comma" => (Key::Comma, Mod::new()),
        "Pause" => (Key::Space, Mod::new().add_fn()),
        "ContextMenu" => (Key::LCtrl, Mod::new().add_fn()),
        "Backquote" => (Key::Backtick, Mod::new()),
        "F1" => (Key::One, Mod::new().add_fn()),
        "F2" => (Key::Two, Mod::new().add_fn()),
        "F3" => (Key::Three, Mod::new().add_fn()),
        "F4" => (Key::Four, Mod::new().add_fn()),
        "F5" => (Key::Five, Mod::new().add_fn()),
        "F6" => (Key::Six, Mod::new().add_fn()),
        "F7" => (Key::Seven, Mod::new().add_fn()),
        "F8" => (Key::Eight, Mod::new().add_fn()),
        "F9" => (Key::Nine, Mod::new().add_fn()),
        "F10" => (Key::Zero, Mod::new().add_fn()),
        "F11" => (Key::Minus, Mod::new().add_fn()),
        "F12" => (Key::Equal, Mod::new().add_fn()),
        "NumpadDivide" => (Key::Slash, Mod::new()),
        "NumpadMultiply" => (Key::Eight, Mod::new().add_shift()),
        "NumpadSubtract" => (Key::Minus, Mod::new()),
        "NumpadAdd" => (Key::Equal, Mod::new().add_shift()),
        "NumLock" => (Key::Compose, Mod::new()),
        "CapsLock" => (Key::Compose, Mod::new()),
        "ScrollLock" => (Key::Compose, Mod::new()),
        "ShiftLeft" => (Key::LShift, Mod::new()),
        "ShiftRight" => (Key::RShift, Mod::new()),
        "ControlLeft" | "MetaLeft" | "OSLeft" => (Key::LCtrl, Mod::new()),
        "ControlRight" | "MetaRight" | "OSRight" => (Key::RCtrl, Mod::new()),
        "AltLeft" => (Key::LAlt, Mod::new()),
        "AltRight" => (Key::RAlt, Mod::new()),
        _ => return None,
    };
    Some(key)
}

/// One type future initialization for key presses and mouse events.
pub(crate) fn init() {
    let localized_input = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("input")
        .unwrap();
    localized_input
        .set_attribute(
            "style",
            "\
        border:0;\
        padding:0;\
        margin:0;\
        position:fixed;\
        top:0;\
        left:0;\
        width:0;\
        height:0;\
    ",
        )
        .unwrap();
    localized_input
        .set_attribute("id", "rust_crate_human__")
        .unwrap();
    let localized_input = localized_input.into();
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_elements_by_tag_name("body")
        .get_with_index(0)
        .unwrap()
        .append_child(&localized_input)
        .unwrap();
    let localized_input: EventTarget = localized_input.into();
    let focus: Closure<dyn Fn()> = Closure::wrap(Box::new(move || {
        let localized_input: HtmlElement = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("rust_crate_human__")
            .unwrap()
            .dyn_into()
            .unwrap();
        localized_input.focus().unwrap();
    }));
    let focus = focus.into_js_value();
    let blur: Closure<dyn Fn()> = Closure::wrap(Box::new(move || {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback(&focus.as_ref().unchecked_ref())
            .unwrap();
    }));
    localized_input
        .add_event_listener_with_callback(
            "blur",
            &blur.as_ref().unchecked_ref(),
        )
        .unwrap();
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
                let localized_input = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id("rust_crate_human__")
                    .unwrap();
                let localized_input: HtmlInputElement =
                    localized_input.dyn_into().unwrap();
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
    localized_input
        .add_event_listener_with_callback(
            "input",
            &input.as_ref().unchecked_ref(),
        )
        .unwrap();
    input.forget();

    #[allow(trivial_casts)] // Actually needed here.
    let key_down: Closure<dyn Fn(KeyboardEvent)> =
        Closure::wrap(Box::new(move |event: KeyboardEvent| {
            // If a input is being `.await`ed, wake the waiting thread.
            if let Some(ref waker) = state().waker {
                // Set future to complete.
                let mut sys_mods = false;
                if let Some(mut keycode) = keycode(&event.code()) {
                    sys_mods = sys_modifier(&event, &mut keycode.1);
                    if key_down_state(keycode.0) {
                        state().input =
                            Some(Input::KeyPress(keycode.1, keycode.0));
                    } else {
                        state().input = None;
                    }
                } else {
                    state().input = None;
                }

                // Prevent web browser from also processing the input.
                event.stop_propagation();
                if !is_printing(&event.code()) || sys_mods {
                    event.prevent_default();
                }

                // Wake the keyboard future.
                waker.wake_by_ref();
            }
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
                let mut sys_mods = false;
                if let Some(mut keycode) = keycode(&event.code()) {
                    sys_mods = sys_modifier(&event, &mut keycode.1);
                    if key_up_state(keycode.0) {
                        state().input =
                            Some(Input::KeyRelease(keycode.1, keycode.0));
                    } else {
                        state().input = None;
                    }
                } else {
                    state().input = None;
                }

                // Prevent web browser from also processing the input.
                event.stop_propagation();
                if !is_printing(&event.code()) || sys_mods {
                    event.prevent_default();
                }

                // Wake the keyboard future.
                waker.wake_by_ref();
            }
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
