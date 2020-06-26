/// W3 Standard gamepad events for PC-style games.
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
}
