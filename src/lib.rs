// Human
// Copyright © 2020-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

//! # Getting Started
//! Add the following to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! human = "0.2"
//! pasts = "0.7"
//! devout = "0.2"
//! ```
//!
//! ```rust,no_run
//! use pasts::{exec, wait};
//! use devout::{log, Tag};
//! use human::Input;
//!
//! const INFO: Tag = Tag::new("Info").show(true);
//!
//! /// The program's shared state.
//! struct State {}
//!
//! /// Event handled by the event loop.
//! enum Event {
//!     Input(Input),
//! }
//!
//! impl State {
//!     /// Event loop.
//!     fn event(&mut self, event: Event) {
//!         match event {
//!             Event::Input(input) => log!(INFO, "Input: {:?}", input),
//!         }
//!     }
//! }
//!
//! /// Start the async executor.
//! fn main() {
//!     let mut state = State {};
//!     let mut input = human::Input::listener();
//!
//!     exec!(state.event(wait! {
//!         Event::Input((&mut input).await),
//!     }));
//! }
//! ```

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/libcala/human/main/res/icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/libcala/human/main/res/icon.svg",
    html_root_url = "https://docs.rs/human"
)]
#![deny(unsafe_code)]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]

mod input;
mod key;
mod btn;

#[cfg(target_arch = "wasm32")]
mod web;

pub use input::{Controller, Input};
pub use key::{Key, Mod};
pub use btn::Btn;
/// Input event from a controller.
///
pub use stick::Event as Controls;
