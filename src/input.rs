/// Input from any human interface device
#[derive(Copy, Clone, Debug)]
pub enum Input<'a> {
    /// User has typed something
    Text(TextInput<'a>),
    /// User has pushed a button on a game controller or emulated
    Game(GameInput),
    /// User has interacted with the user interface (usually a GUI, but can be
    /// auditory or other)
    Ui(UiInput),
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
    Tab,
    /// Shift-Tab/BumperL (1-D focus selector previous)
    UnTab,
    /// ArrowUp/DpadUp/JoystickUp (2-D Focus selector up)
    Up,
    /// ArrowDown/DpadDown/JoystickDown (2-D Focus selector down)
    Down,
    /// ArrowLeft/DpadLeft/JoystickLeft (2-D Focus selector left)
    Left,
    /// ArrowRight/DpadRight/JoystickRight (2-D Focus selector right)
    Right,
}

/// Game input, W3 Standard gamepad with extensions events for PC-style games.
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
