/// Input keycode for a key on a keyboard.
/// 
/// The first 64 inputs are represented using numbers 0-63 - they are the
/// keyboard keys without the function key pressed.  The next 64 inputs are
/// represented using the numbers 64-127.
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Key {
    // Start With Basic Keyboard - Without Function Key
    /// Also known as the ESCAPE key.
    Back = 0x00u8,
    /// Numeric 1 on either top row, or numpad with num-lock enabled
    One = 0x01,
    /// Numeric 2 on either top row, or numpad with num-lock enabled
    Two = 0x02,
    /// Numeric 3 on either top row, or numpad with num-lock enabled
    Three = 0x03,
    /// Numeric 4 on either top row, or numpad with num-lock enabled
    Four = 0x04,
    /// Numeric 5 on either top row, or numpad with num-lock enabled
    Five = 0x05,
    /// Numeric 6 on either top row, or numpad with num-lock enabled
    Six = 0x06,
    /// Numeric 7 on either top row, or numpad with num-lock enabled
    Seven = 0x07,
    /// Numeric 8 on either top row, or numpad with num-lock enabled
    Eight = 0x08,
    /// Numeric 9 on either top row, or numpad with num-lock enabled
    Nine = 0x09,
    /// Numeric 0 on either top row, or numpad with num-lock enabled
    Zero = 0x0A,
    /// Minus / Underscore Key
    Minus = 0x0B,
    /// Equal Sign / Plus Key
    Equal = 0x0C,
    /// Backtick / Tilde Key
    Backtick = 0x0D,
    /// Lang Key (Also Insert Key).
    Lang = 0x0E,
    /// Tab / Indent
    Tab = 0x10,
    /// Q (may be named by a different glyph depending on language of user).
    Q = 0x11,
    /// W (may be named by a different glyph depending on language of user).
    W = 0x12,
    /// E (may be named by a different glyph depending on language of user).
    E = 0x13,
    /// R (may be named by a different glyph depending on language of user).
    R = 0x14,
    /// T (may be named by a different glyph depending on language of user).
    T = 0x15,
    /// Y (may be named by a different glyph depending on language of user).
    Y = 0x16,
    /// U (may be named by a different glyph depending on language of user).
    U = 0x17,
    /// I (may be named by a different glyph depending on language of user).
    I = 0x18,
    /// U (may be named by a different glyph depending on language of user).
    O = 0x19,
    /// I (may be named by a different glyph depending on language of user).
    P = 0x1A,
    /// [ (may be named by a different glyph depending on language of user).
    BracketOpen = 0x1B,
    /// ] (may be named by a different glyph depending on language of user).
    BracketClose = 0x1C,
    /// Delete (Also Backspace).
    Delete = 0x1D,
    /// Env (Also known as: Win, Super, Cmd, Search) Key
    Env = 0x20,
    /// A (may be named by a different glyph depending on language of user).
    A = 0x21,
    /// S (may be named by a different glyph depending on language of user).
    S = 0x22,
    /// D (may be named by a different glyph depending on language of user).
    D = 0x23,
    /// F (may be named by a different glyph depending on language of user).
    F = 0x24,
    /// G (may be named by a different glyph depending on language of user).
    G = 0x25,
    /// H (may be named by a different glyph depending on language of user).
    H = 0x26,
    /// J (may be named by a different glyph depending on language of user).
    J = 0x27,
    /// K (may be named by a different glyph depending on language of user).
    K = 0x28,
    /// L (may be named by a different glyph depending on language of user).
    L = 0x29,
    /// ; (may be named by a different glyph depending on language of user).
    Semicolon = 0x2A,
    /// ' (may be named by a different glyph depending on language of user).
    Apostrophe = 0x2B,
    /// Enter (Also Return).
    Enter = 0x2C,
    /// Left Shift Key
    LShift = 0x30,
    /// Z (may be named by a different glyph depending on language of user).
    Z = 0x32,
    /// X (may be named by a different glyph depending on language of user).
    X = 0x33,
    /// C (may be named by a different glyph depending on language of user).
    C = 0x34,
    /// V (may be named by a different glyph depending on language of user).
    V = 0x35,
    /// B (may be named by a different glyph depending on language of user).
    B = 0x36,
    /// N (may be named by a different glyph depending on language of user).
    N = 0x37,
    /// M (may be named by a different glyph depending on language of user).
    M = 0x38,
    /// , (may be named by a different glyph depending on language of user).
    Comma = 0x39,
    /// . (may be named by a different glyph depending on language of user).
    Period = 0x3A,
    /// / (may be named by a different glyph depending on language of user).
    Slash = 0x3B,
    /// \ (may be named by a different glyph depending on language of user).
    Backslash = 0x3C,
    /// Up Arrow
    Up = 0x3D,
    /// Right Shift Key
    RShift = 0x3E,
    /// Left Ctrl / Control Key
    LCtrl = 0x0F,
    /// Left Alt Key
    LAlt = 0x1E,
    /// Space Key (Also Left Thumb Key)
    Space = 0x1F,
    /// Right Alt Key
    RAlt = 0x2D,
    /// Right Ctrl / Control Key
    RCtrl = 0x2E,
    /// Left Arrow
    Left = 0x2F,
    /// Down Arrow
    Down = 0x31,
    /// Right Arrow
    Right = 0x3F,

    // Next Is Keyboard Extensions - With Function Key
    /// The menu key (Fn + Back).
    Menu = 0x40,
    /// F1 Function Key (Fn + 1)
    F1 = 0x41,
    /// F2 Function Key (Fn + 2)
    F2 = 0x42,
    /// F3 Function Key (Fn + 3)
    F3 = 0x43,
    /// F4 Function Key (Fn + 4)
    F4 = 0x44,
    /// F5 Function Key (Fn + 5)
    F5 = 0x45,
    /// F6 Function Key (Fn + 6)
    F6 = 0x46,
    /// F7 Function Key (Fn + 7)
    F7 = 0x47,
    /// F8 Function Key (Fn + 8)
    F8 = 0x48,
    /// F9 Function Key (Fn + 9)
    F9 = 0x49,
    /// F10 Function Key (Fn + 0)
    F10 = 0x4A,
    /// F11 Function Key (Fn + Minus)
    F11 = 0x4B,
    /// F12 Function Key (Fn + Equal)
    F12 = 0x4C,
    /// Previous Multimedia Key (Fn + Backtick)
    PrevTrack = 0x4D,
    /// Next Multimedia Key (Fn + Lang)
    NextTrack = 0x4E,
    /// Toggle/Select Display Configuration (Fn + Tab)
    Display = 0x50,
    /// Disable All Capture Hardware (Web Cam and Microphone, Fn + Q)
    Disable = 0x51,
    /// Enable/Disable Webcam (Fn + W)
    Cam = 0x52,
    /// Decrease webcam exposure (Fn + E)
    DecreaseExposure = 0x53,
    /// Increase webcam exposure (Fn + R)
    IncreaseExposure = 0x54,
    /// Fn + T
    DecreaseCamSettingA = 0x55,
    /// Fn + Y
    IncreaseCamSettingA = 0x56,
    /// Fn + U
    DecreaseCamSettingB = 0x57,
    /// Fn + I
    IncreaseCamSettingB = 0x58,
    /// Scroll Lock (Fn + O)
    ScrollLk = 0x59,
    /// Airplane Mode (Fn + P)
    PrintScreen = 0x5A,
    /// Airplane Mode On (Fn + [)
    AirplaneOn = 0x5B,
    /// Airplane Mode Off (Fn + ])
    AirplaneOff = 0x5C,
    /// Delete Next (Fn + Delete)
    Del = 0x5D,
    /// The Caps Lock Key (Fn + Env)
    CapsLk = 0x60,
    /// Enable Both Web Cam and Microphone (Fn + A)
    Enable = 0x61,
    /// Louder - Also Volume Up (Fn + S)
    Louder = 0x62,
    /// Dim - Also Brightness Down (Fn + D)
    Dim = 0x63,
    /// Brighten - Also Brightness Up (Fn + F)
    Brighten = 0x64,
    /// The emulation focus key (Fn + F)
    Focus = 0x65,
    /// The Terminal Key
    Terminal = 0x6A,
    /// The Take Picture App Launcher (with webcam) Key
    Picture = 0x6B,
    /// The Calculator Key
    Calculator = 0x6C,
    /// The Function Key
    Fn = 0x70,
    /// Mute (Fn + Z)
    Mute = 0x72,
    /// Quieter - Also Volume Down (Fn + X)
    Quieter = 0x73,
    /// Stop (Fn + C)
    Stop = 0x74,
    /// Change Microphone (Fn + V)
    MicSrc = 0x75,
    /// Change Speaker (Fn + B)
    SpkSrc = 0x76,
    /// Change Camera (Fn + N)
    CamSrc = 0x77,
    /// Music Player Launcher (Fn + M)
    Music = 0x78,
    /// Sound Settings Launcher (Fn + Comma)
    Sound = 0x79,
    /// Enable/Disable Internal Screen (Fn + Period)
    Internet = 0x7A,
    /// Help Launcher (Fn + Slash)
    Help = 0x7B,
    /// Customizable App launcher (Fn + Backslash)
    Email = 0x7C,
    /// Up Arrow
    PgUp = 0x7D,
    /// The Pause Key
    Pause = 0x7E,
    /// Additional Localized Input Key Not Handled by the Lang Key
    Input = 0x5E,
    /// Compose Key (Also AltGr, Right Thumb Key)
    Compose = 0x5F,
    /// Home Key
    Home = 0x6F,
    /// Page Down
    PgDn = 0x71,
    /// End Key
    End = 0x7F,

    // Next Is Mouse Buttons Click
    /// Extra Mouse Button Click
    Extra = 0x4F,
    /// Left Mouse Button Click
    Click = 0x66,
    /// Right Mouse Button Click
    Option = 0x67,
    /// Scroll Wheel Click
    Scroll = 0x68,
    /// DPI Mouse Button Click
    Dpi = 0x69,
    /// Prev / Back Mouse Button Click
    Prev = 0x6D,
    /// Next / Forward Mouse Buttton Click
    Next = 0x6E,
}
