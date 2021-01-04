# Changelog
All notable changes to "human" will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://github.com/AldaronLau/semver#a-guide-to-semver).

## [0.2.0] - 2021-01-03
### Added
 - Web input support.
 - `Controller` struct.
 - `Mod` struct for keyboard modifiers.
 - `Btn` enum for mouse buttons.
 - `Key` enum for key of a keyboard.

### Changed
 - Replace `GameInput` enum with `Controls` enum.
 - Replace all of the variants of the `Input` enum.

### Removed
 - `input()`, use `Input::listener()` instead.
 - `set_mode()`, `mode_gamepad()`, `mode_keyboard()` and `mode_pointer()`
   functions.
 - `renumber()` function.
 - `rumble()` function, use `Controller::rumble()` instead.

## [0.1.0] - 2020-07-13
### Added
 - `input()`
 - Getters for modes
 - `renumber()` for renumbering gamepads
 - `rumble()` for rumble support
 - `set_mode()` to set the modes of input devices
 - `Input` enumeration
 - `Mode` enumeration
