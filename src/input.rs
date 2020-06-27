use std::cell::RefCell;
use pasts::prelude::*;
use stick::{Event, Hub, Pad};

/// Input from any human interface device
#[derive(Copy, Clone, Debug)]
pub enum Input<'a> {
    /// User has typed something
    Text(TextInput<'a>),
    /// User has pushed a button on a game controller or emulated
    Game(usize, GameInput),
    /// User has interacted with the user interface (usually a GUI, but can be
    /// auditory or other)
    Ui(UiInput),
    /// User has requested to exit
    Exit,
}

/// User interface input; touchscreen, trackpad, mouse, keyboard or gamepad
/// pointer.
#[derive(Copy, Clone, Debug)]
pub enum UiInput {
    /// ScrollWheelY (Vertical scrolling)
    ScrollV(f32),
    /// ScrollWheelX / Shift-ScrollWheelY (Horizontal scrolling)
    ScrollH(f32),
    /// MouseMove
    Move(f32, f32),
    /// LeftClick / Touch / Space Key / Enter Key
    Push(f32, f32),
    /// LeftClick / Touch Held + Movement
    Drag(f32, f32),
    /// Tab/BumperR (1-D focus selector next)
    Next,
    /// Shift-Tab/BumperL (1-D focus selector previous)
    Prev,
    /// ArrowUp/DpadUp/JoystickUp (2-D Focus selector up)
    Up,
    /// ArrowDown/DpadDown/JoystickDown (2-D Focus selector down)
    Down,
    /// ArrowLeft/DpadLeft/JoystickLeft (2-D Focus selector left)
    Left,
    /// ArrowRight/DpadRight/JoystickRight (2-D Focus selector right)
    Right,
}

/// Game input, W3 Standard gamepad with extensions events for PC/Console-style
/// games.
#[derive(Copy, Clone, Debug)]
pub enum GameInput {
    /// G Key or Forward Button/Press Start Button
    Menu,
    /// Escape Key or Back Button/Hold Start Button
    Pause,
    /// Left Click/Enter Key or A/Circle Button
    A(bool),
    /// Shift Key or B Button
    B(bool),
    /// Right Click or X/Y/Square Button
    H(bool),
    /// Space Key or X/Y/Triangle Button
    V(bool),
    /// D-Pad or Arrow Key Up or R Key
    Up(bool),
    /// D-Pad or Arrow Key Down or F Key
    Down(bool),
    /// D-Pad or Arrow Key Left or Scroll Down
    Left(bool),
    /// D-Pad or Arrow Key Right or Scroll Up
    Right(bool),
    /// Ctrl Key or Left Bumper Trigger
    TriggerL(f64),
    /// Alt Key or Right Bumper Trigger
    TriggerR(f64),
    /// U Key or Left Bumper Button
    BumperL(bool),
    /// E Key or Right Bumper Button
    BumperR(bool),
    /// Tab Key or Left Joystick Push
    JoyPush(bool),
    /// Middle Click or Right Joystick Push
    CamPush(bool),
    /// A/D Keys Axis or Left Joystick X
    JoyX(f64),
    /// W/S Keys Axis or Left Joystick Y
    JoyY(f64),
    /// Mouse X delta or Right Joystick X
    CamX(f64),
    /// Mouse Y delta or Right Joystick Y
    CamY(f64),
    /// Inventory slot jumping (only available via number keys 1-10(11-,12=))
    Slot(u8),
    /// Combo Action Button
    ComboLeft(bool),
    /// Combo Action Button
    ComboRight(bool),
    /// Combo Action Button
    ComboLeftPinky(bool),
    /// Combo Action Button
    ComboRightPinky(bool),
}

/// Typing input Ctrl refers to Cmd on MacOS.
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
#[allow(variant_size_differences)]
pub enum TextInput<'a> {
    /// User has entered some text via keyboard or paste
    Text(&'a str),
    /// Backspace / Shift-Delete (Delete previous character)
    Backspace,
    /// Delete / Shift-Backspace (Delete next character)
    Delete,
    /// User has entered compose key (show visually somehow, no other action
    /// necessary)
    Compose(bool),
    /// Enter (Newline, nextline or submit)
    Enter,
    /// Shift-Enter (Previous line - Useful for spreadsheet and music software)
    Retract,
    /// Tab (Indent or next)
    Tab,
    /// Shift-Tab (UnIndent or previous)
    UnTab,
    /// ArrowUp (Move cursor up)
    Up,
    /// ArrowDown (Move cursor down)
    Down,
    /// ArrowLeft (Move cursor left)
    Left,
    /// ArrowRight (Move cursor right)
    Right,
    /// Shift-ArrowUp (Cursor select up)
    SelectUp,
    /// Shift-ArrowDown (Cursor select down)
    SelectDown,
    /// Shift-ArrowLeft (Cursor select left)
    SelectLeft,
    /// Shift-ArrowRight (Cursor select right)
    SelectRight,
    /// Escape key or back button
    Exit,
    /// Ctrl-C
    Copy,
    /// Alt-C (Ctrl-Shift-C)
    ClipboardView,
    /// Ctrl-V
    Paste,
    /// Alt-V (Ctrl-Shift-V)
    PasteUnformat,
    /// Ctrl-X
    Cut,
    /// Alt-X (Ctrl-Shift-X)
    Swap,
    /// Ctrl-A
    SelectAll,
    /// Ctrl-D
    DeleteLine,
    /// Ctrl-F
    Find,
    /// Alt-F
    FindReplace,
    /// Ctrl-G
    GotoLine,
    /// Ctrl-Z
    Undo,
    /// Alt-Z (Ctrl-Y, Ctrl-Shift-Z)
    Redo,
    /// Ctrl-S
    Sync,
    /// Alt-S (Ctrl-Shift-S)
    Share,
    /// Home (Beginning of line)
    Home,
    /// End (End of Line)
    End,
    /// PageUp (Up one page)
    PageUp,
    /// PageDown (Down one page)
    PageDown,
    /// Ctrl-U (Make selection underlined)
    Underline,
    /// Ctrl-I (Make selection italic)
    Italic,
    /// Ctrl-B (Make selection bold)
    Bold,
    /// Alt-ArrowLeft (Swap word left)
    SwapLeft,
    /// Alt-ArrowRight (Swap word right)
    SwapRight,
    /// Alt-ArrowUp (swap line up)
    SwapUp,
    /// Alt-ArrowDown (swap line down)
    SwapDown,
    /// Ctrl-Delete / Alt-Backspace (Delete until space)
    DeleteWord,
    /// Ctrl-Backspace / Alt-Delete (Backspace until space)
    BackspaceWord,
    /// Ctrl-ArrowLeft
    WordLeft,
    /// Ctrl-ArrowRight
    WordRight,
    /// Ctrl-ArrowUp (Up and Home at once)
    UpHome,
    /// Ctrl-ArrowDown (Down and End at once)
    DownEnd,
    /// Ctrl-N (New)
    New,
    /// Ctrl-O (Open)
    Open,
    /// Alt-N / Ctrl-Shift-N (New With Template)
    Template,
}

enum Pads {
    Gameplay(Vec<Option<Pad>>),
    Renumbering(Vec<Pad>),
}

impl Pads {
    // Is renumbering on
    fn renumber(&self) -> bool {
        match self {
            Pads::Gameplay(_) => false,
            Pads::Renumbering(_) => true,
        }
    }
}

// Input global context.
struct Context {
    hub: Hub,
    pads: Pads,
}

thread_local! {
    static CONTEXT: RefCell<Option<Box<Context>>> = RefCell::new(Some(Box::new(Context {
        hub: Hub::new(),
        pads: Pads::Gameplay(Vec::new()),
    })));
}

/// Turn gamepad renumbering on or off.
///
/// This is useful for the start of a video game when you don't want gaps
/// between player numbers.  By default it's off.  During gameplay you should
/// turn it back off so that the players don't change what character they are
/// controlling.  For single-player games or multiplayer over multiple devices
/// you don't have to worry about this setting.
pub fn renumber(on: bool) {
    let mut cx = CONTEXT.with(|cx| cx.borrow_mut().take().expect("HIDs can't be used in multiple places at once"));

    if on != cx.pads.renumber() {
        // Toggle
        match cx.pads {
            Pads::Gameplay(pads) => {
                let mut new = Vec::new();
                for pad in pads {
                    if let Some(pad) = pad {
                        new.push(pad);
                    }
                }
                cx.pads = Pads::Renumbering(new);
            }
            Pads::Renumbering(pads) => {
                let mut new = Vec::new();
                for pad in pads {
                    new.push(Some(pad));
                }
                cx.pads = Pads::Gameplay(new);
            }
        }
    }
    
    CONTEXT.with(|new| *new.borrow_mut() = Some(cx));
}

/// Get user input from terminal and gamepads
pub async fn input<'a>() -> Input<'a> {
    let mut cx = CONTEXT.with(|cx| cx.borrow_mut().take().expect("HIDs can't be used in multiple places at once"));

    let input = 'input: loop {
        let mut pads_fut = match cx.pads {
            Pads::Gameplay(ref mut pads) => pads.select(),
            Pads::Renumbering(ref mut pads) => pads.select(),
        };

        match [cx.hub.fut(), pads_fut.fut()].select().await.1 {
            (_, Event::Connect(new)) => {
                match cx.pads {
                    Pads::Gameplay(ref mut pads) => {
                        for pad in pads.iter_mut() {
                            if pad.is_none() {
                                *pad = Some(*new);
                                continue 'input;
                            }
                        }
                        pads.push(Some(*new));
                    },
                    Pads::Renumbering(ref mut pads) => pads.push(*new),
                }
            }
            (id, Event::Disconnect) => {
                match cx.pads {
                    Pads::Gameplay(ref mut pads) => *pads.get_mut(id).unwrap() = None,
                    Pads::Renumbering(ref mut pads) => { pads.swap_remove(id); }
                }
            }
            (id, Event::Home(true)) => {
                match cx.pads {
                    Pads::Gameplay(_) => if id == 0 {
                        break 'input Input::Exit;
                    }
                    Pads::Renumbering(ref mut pads) => {
                        let pad = pads.swap_remove(id);
                        pads.push(pad);
                    }
                }
            }
            (id, event) => {
                use Event::*;
                use Input::Game;
                break 'input match event {
                    Prev(true) => Game(id, GameInput::Pause),
                    Next(true) => Game(id, GameInput::Menu),
                    ActionA(p) => Game(id, GameInput::A(p)),
                    ActionB(p) => Game(id, GameInput::B(p)),
                    ActionH(p) => Game(id, GameInput::H(p)),
                    ActionV(p) => Game(id, GameInput::V(p)),
                    DpadUp(p) => Game(id, GameInput::Up(p)),
                    DpadDown(p) => Game(id, GameInput::Down(p)),
                    DpadLeft(p) => Game(id, GameInput::Left(p)),
                    DpadRight(p) => Game(id, GameInput::Right(p)),
                    TriggerL(p) => Game(id, GameInput::TriggerL(p)),
                    TriggerR(p) => Game(id, GameInput::TriggerR(p)),
                    BumperL(p) => Game(id, GameInput::BumperL(p)),
                    BumperR(p) => Game(id, GameInput::BumperR(p)),
                    JoyX(p) => Game(id, GameInput::JoyX(p)),
                    JoyY(p) => Game(id, GameInput::JoyY(p)),
                    CamX(p) | JoyZ(p) => Game(id, GameInput::CamX(p)),
                    CamY(p) => Game(id, GameInput::CamY(p)),
                    JoyPush(p) => Game(id, GameInput::JoyPush(p)),
                    CamPush(p) => Game(id, GameInput::CamPush(p)),
                    PaddleRight(p) => Game(id, GameInput::ComboRight(p)),
                    PaddleLeft(p) => Game(id, GameInput::ComboLeft(p)),
                    PaddleRightPinky(p) => Game(id, GameInput::ComboRightPinky(p)),
                    PaddleLeftPinky(p) => Game(id, GameInput::ComboLeftPinky(p)),
                    _ => continue 'input,
                }
            }
        }
    };
    
    CONTEXT.with(|new| *new.borrow_mut() = Some(cx));
    input
}

/// Send rumble effect to a mobile device or gamepad(s).
///
/// If id is `Some` then the gamepad with the ID specified will rumble.  If id
/// is `None` then all of the gamepads will rumble.  The rumbling will continue
/// until you call this function again with `power` set to `0.0`.  Maximum
/// `power` is `1.0`.  Use `None` for mobile devices.  Make sure to use caution
/// when testing to ensure that no nearby fragile items might break from the
/// rumble effect.
///
/// # Notes
/// - `power` will automatically be clamped between `0.0` and `1.0`.
pub fn rumble(id: Option<usize>, power: f32) {
    let mut cx = CONTEXT.with(|cx| cx.borrow_mut().take().expect("HIDs can't be used in multiple places at once"));

    if let Some(id) = id {
        match cx.pads {
            Pads::Gameplay(ref mut pads) => {
                if let Some(Some(ref mut pad)) = pads.get_mut(id) {
                    pad.rumble(power);
                }
            }
            Pads::Renumbering(ref mut pads) => {
                if let Some(ref mut pad) = pads.get_mut(id) {
                    pad.rumble(power);
                }
            }
        }
    } else {
        match cx.pads {
            Pads::Gameplay(ref mut pads) => {
                for pad in pads.iter_mut() {
                    if let Some(ref mut pad) = pad {
                        pad.rumble(power);
                    }
                }
            }
            Pads::Renumbering(ref mut pads) => {
                for pad in pads.iter_mut() {
                    pad.rumble(power);
                }
            }
        }
    }
    
    CONTEXT.with(|new| *new.borrow_mut() = Some(cx));
}
