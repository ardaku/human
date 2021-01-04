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
    AddEventListenerOptions, Event, EventTarget, HtmlElement, HtmlInputElement,
    InputEvent, KeyboardEvent, MouseEvent, WheelEvent,
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
static KEYBOARD_STATE_FN: AtomicU64 = AtomicU64::new(0);

fn key_up_state(key: Key) -> bool {
    if key as i8 >= 64 {
        let bitflag = 1 << (key as i8 - 64);
        let new = KEYBOARD_STATE_FN.fetch_and(!bitflag, SeqCst);
        let old = KEYBOARD_STATE_FN.load(SeqCst);

        old != new
    } else {
        let bitflag = 1 << (key as i8);
        let new = KEYBOARD_STATE.fetch_and(!bitflag, SeqCst);
        let old = KEYBOARD_STATE.load(SeqCst);

        old != new
    }
}

fn key_down_state(key: Key) -> bool {
    if key as i8 >= 64 {
        let bitflag = 1 << (key as i8 - 64);
        let new = KEYBOARD_STATE_FN.fetch_or(bitflag, SeqCst);
        let old = KEYBOARD_STATE_FN.load(SeqCst);

        old != new
    } else {
        let bitflag = 1 << (key as i8);
        let new = KEYBOARD_STATE.fetch_or(bitflag, SeqCst);
        let old = KEYBOARD_STATE.load(SeqCst);

        old != new
    }
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

fn ptr_modifier(event: &MouseEvent) -> Mod {
    let mut mods = Mod::new();

    let ctrl =
        event.get_modifier_state("Control") || event.get_modifier_state("Meta");
    let alt = event.get_modifier_state("Alt");
    let shift = event.get_modifier_state("Shift");

    if ctrl {
        mods = mods.add_ctrl();
    }

    if alt {
        mods = mods.add_alt();
    }

    if shift {
        mods = mods.add_shift();
    }

    mods
}

fn key_modifier(event: &KeyboardEvent, mods: &mut Mod) -> bool {
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
        "Enter" | "NumpadEnter" => (Key::Enter, Mod::new()),
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
        "Backspace" => (Key::Backspace, Mod::new()),
        "Delete" => (Key::Delete, Mod::new()),
        "Escape" => (Key::Back, Mod::new()),
        "Minus" => (Key::Minus, Mod::new()),
        "Equal" => (Key::Equal, Mod::new()),
        "Insert" => (Key::Insert, Mod::new()),
        "PageUp" => (Key::PageUp, Mod::new()),
        "PageDown" => (Key::PageDown, Mod::new()),
        "Home" => (Key::Home, Mod::new()),
        "End" => (Key::End, Mod::new()),
        "Backslash" => (Key::Backslash, Mod::new()),
        "BracketLeft" => (Key::BracketOpen, Mod::new()),
        "BracketRight" => (Key::BracketClose, Mod::new()),
        "Quote" => (Key::Apostrophe, Mod::new()),
        "Semicolon" => (Key::Semicolon, Mod::new()),
        "Slash" => (Key::Slash, Mod::new()),
        "Period" | "NumpadDecimal" => (Key::Period, Mod::new()),
        "Comma" => (Key::Comma, Mod::new()),
        "Pause" => (Key::Pause, Mod::new()),
        "ContextMenu" => (Key::Menu, Mod::new()),
        "Backquote" => (Key::Backtick, Mod::new()),
        "F1" => (Key::F1, Mod::new()),
        "F2" => (Key::F2, Mod::new()),
        "F3" => (Key::F3, Mod::new()),
        "F4" => (Key::F4, Mod::new()),
        "F5" => (Key::F5, Mod::new()),
        "F6" => (Key::F6, Mod::new()),
        "F7" => (Key::F7, Mod::new()),
        "F8" => (Key::F8, Mod::new()),
        "F9" => (Key::F9, Mod::new()),
        "F10" => (Key::F10, Mod::new()),
        "F11" => (Key::F11, Mod::new()),
        "F12" => (Key::F12, Mod::new()),
        "NumpadDivide" => (Key::Slash, Mod::new()),
        "NumpadMultiply" => (Key::Eight, Mod::new().add_shift()),
        "NumpadSubtract" => (Key::Minus, Mod::new()),
        "NumpadAdd" => (Key::Equal, Mod::new().add_shift()),
        "NumLock" => (Key::Compose, Mod::new()),
        "CapsLock" => (Key::CapsLock, Mod::new()),
        "ScrollLock" => (Key::Compose, Mod::new()),
        "ShiftLeft" => (Key::LShift, Mod::new()),
        "ShiftRight" => (Key::RShift, Mod::new()),
        "ControlLeft" | "MetaLeft" => (Key::LCtrl, Mod::new()),
        "ControlRight" | "MetaRight" => (Key::RCtrl, Mod::new()),
        "OSLeft" | "OSRight" => (Key::Env, Mod::new()),
        "AltLeft" => (Key::LAlt, Mod::new()),
        "AltRight" => (Key::RAlt, Mod::new()),
        _ => return None,
    };
    Some(key)
}

/// Convert into pixels.
pub(crate) fn delta(mode: u32, value: f64) -> f64 {
    match mode {
        0x00 => value,
        0x01 => value * 16.0,
        0x02 => {
            value
                * web_sys::window()
                    .unwrap()
                    .inner_height()
                    .unwrap()
                    .as_f64()
                    .unwrap()
        }
        _ => unreachable!(),
    }
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
                    sys_mods = key_modifier(&event, &mut keycode.1);
                    if key_down_state(keycode.0) {
                        state().input =
                            Some(Input::Key(keycode.1, keycode.0, true));
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
                    sys_mods = key_modifier(&event, &mut keycode.1);
                    if key_up_state(keycode.0) {
                        state().input =
                            Some(Input::Key(keycode.1, keycode.0, false));
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

    #[allow(trivial_casts)] // Actually needed here.
    let mouse_down: Closure<dyn Fn(MouseEvent)> =
        Closure::wrap(Box::new(move |event: MouseEvent| {
            let mods = ptr_modifier(&event);

            // If a input is being `.await`ed, wake the waiting thread.
            if let Some(ref waker) = state().waker {
                // Set future to complete.
                state().input = match (mods, event.button()) {
                    (m, 0) if m.ctrl() => Some(Input::Context(true)),
                    (m, 1) if m.ctrl() => Some(Input::Context(true)),
                    (m, 2) if m.ctrl() => Some(Input::Central(true)),
                    (m, 0) if m.alt() => Some(Input::Central(true)),
                    (m, 1) if m.alt() => Some(Input::Context(true)),
                    (m, 2) if m.alt() => Some(Input::Central(true)),
                    (m, 2) if m.shift() => None,
                    (m, _) if m.shift() => Some(Input::SelArea(true)),
                    (_, 0) => Some(Input::Pointer(true)),
                    (_, 1) => Some(Input::Central(true)),
                    (_, 2) => Some(Input::Context(true)),
                    _ => None,
                };

                // Wake the keyboard future.
                waker.wake_by_ref();
            }
        }));
    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback(
            "mousedown",
            mouse_down.as_ref().unchecked_ref(),
        )
        .unwrap();
    mouse_down.forget();

    #[allow(trivial_casts)] // Actually needed here.
    let mouse_up: Closure<dyn Fn(MouseEvent)> =
        Closure::wrap(Box::new(move |event: MouseEvent| {
            let mods = ptr_modifier(&event);

            // If a input is being `.await`ed, wake the waiting thread.
            if let Some(ref waker) = state().waker {
                // Set future to complete.
                state().input = match (mods, event.button()) {
                    (m, 0) if m.ctrl() => Some(Input::Context(false)),
                    (m, 1) if m.ctrl() => Some(Input::Context(false)),
                    (m, 2) if m.ctrl() => Some(Input::Central(false)),
                    (m, 0) if m.alt() => Some(Input::Central(false)),
                    (m, 1) if m.alt() => Some(Input::Context(false)),
                    (m, 2) if m.alt() => Some(Input::Central(false)),
                    (m, 2) if m.shift() => None,
                    (m, _) if m.shift() => Some(Input::SelArea(false)),
                    (_, 0) => Some(Input::Pointer(false)),
                    (_, 1) => Some(Input::Central(false)),
                    (_, 2) => Some(Input::Context(false)),
                    _ => None,
                };

                // Wake the keyboard future.
                waker.wake_by_ref();
            }
        }));
    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback(
            "mouseup",
            mouse_up.as_ref().unchecked_ref(),
        )
        .unwrap();
    mouse_up.forget();

    #[allow(trivial_casts)] // Actually needed here.
    let context_menu: Closure<dyn Fn(Event)> =
        Closure::wrap(Box::new(move |event: Event| {
            // If a input is being `.await`ed,
            if state().waker.is_some() {
                // Ignore these events, and don't let the browser process them.
                event.stop_propagation();
                event.prevent_default();
            }
        }));
    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback(
            "contextmenu",
            context_menu.as_ref().unchecked_ref(),
        )
        .unwrap();
    context_menu.forget();

    #[allow(trivial_casts)] // Actually needed here.
    let wheel: Closure<dyn Fn(WheelEvent)> =
        Closure::wrap(Box::new(move |event: WheelEvent| {
            let mods = ptr_modifier(&event);
            let width = web_sys::window()
                .unwrap()
                .inner_width()
                .unwrap()
                .as_f64()
                .unwrap();
            let delta_mode = event.delta_mode();

            // If a input is being `.await`ed, wake the waiting thread.
            if let Some(ref waker) = state().waker {
                let x = delta(delta_mode, event.delta_x()) / width;
                if x.abs() > f64::EPSILON {
                    state().input = match mods {
                        m if m.none() => Some(Input::ScrollX(x)),
                        m if m.ctrl() => Some(Input::Zoom(1.0 + x)),
                        m if m.alt() || m.func() => Some(Input::Rotate(x)),
                        _ => Some(Input::ScrollY(x)),
                    };

                    // Wake the keyboard future.
                    waker.wake_by_ref();
                }
            }
            // If a input is being `.await`ed, wake the waiting thread.
            if let Some(ref waker) = state().waker {
                let y = delta(delta_mode, event.delta_y()) / width;
                if y.abs() > f64::EPSILON {
                    state().input = match mods {
                        m if m.none() => Some(Input::ScrollY(y)),
                        m if m.ctrl() => Some(Input::Zoom(1.0 - y)),
                        m if m.alt() || m.func() => Some(Input::Rotate(y)),
                        _ => Some(Input::ScrollX(y)),
                    };

                    // Wake the keyboard future.
                    waker.wake_by_ref();
                }
            }

            // Prevent zoom and scroll.
            event.stop_propagation();
            event.prevent_default();
        }));
    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback_and_add_event_listener_options(
            "wheel",
            wheel.as_ref().unchecked_ref(),
            &AddEventListenerOptions::new().passive(false),
        )
        .unwrap();
    wheel.forget();

    #[allow(trivial_casts)] // Actually needed here.
    let mouse_move: Closure<dyn Fn(MouseEvent)> =
        Closure::wrap(Box::new(move |event: MouseEvent| {
            let width = web_sys::window()
                .unwrap()
                .inner_width()
                .unwrap()
                .as_f64()
                .unwrap()
                - 1.0;

            // If a input is being `.await`ed, wake the waiting thread.
            if let Some(ref waker) = state().waker {
                let x = event.client_x() as f64 / width;
                if x.abs() > f64::EPSILON {
                    state().input = Some(Input::PointerX(x));

                    // Wake the keyboard future.
                    waker.wake_by_ref();
                }
            }
            // If a input is being `.await`ed, wake the waiting thread.
            if let Some(ref waker) = state().waker {
                let y = event.client_y() as f64 / width;
                if y.abs() > f64::EPSILON {
                    state().input = Some(Input::PointerY(y));

                    // Wake the keyboard future.
                    waker.wake_by_ref();
                }
            }
        }));
    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback(
            "mousemove",
            mouse_move.as_ref().unchecked_ref(),
        )
        .unwrap();
    mouse_move.forget();

    #[allow(trivial_casts)] // Actually needed here.
    let mouse_leave: Closure<dyn Fn(MouseEvent)> =
        Closure::wrap(Box::new(move |_event: MouseEvent| {
            // If a input is being `.await`ed, wake the waiting thread.
            if let Some(ref waker) = state().waker {
                state().input = Some(Input::PointerLeave);
                // Wake the keyboard future.
                waker.wake_by_ref();
            }
        }));
    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback(
            "mouseout",
            mouse_leave.as_ref().unchecked_ref(),
        )
        .unwrap();
    mouse_leave.forget();
}
