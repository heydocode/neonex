use bevy::input::keyboard::{KeyCode, KeyboardInput};
use uefi::{
    Char16, ResultExt,
    boot::{self, ScopedProtocol},
    proto::console::{
        self,
        text::{Key, ScanCode},
    },
};

pub struct UefiInputReader {
    input: ScopedProtocol<console::text::Input>,
}

impl UefiInputReader {
    pub fn new(input: ScopedProtocol<console::text::Input>) -> Self {
        Self { input }
    }

    pub fn read_event(&mut self) -> uefi::Result<Option<KeyCode>> {
        // Pause until a keyboard event occurs.
        let mut events = [self.input.wait_for_key_event().unwrap()];
        boot::wait_for_event(&mut events).discard_errdata()?;
        let event = self.input.read_key()?;
        let code = match event {
            Some(Key::Printable(c)) if c == '\r' => Some(KeyCode::Enter),
            Some(Key::Printable(c)) => {
                if c.is_ascii() {
                    Some(match char::from(c) {
                        'a' | 'A' => KeyCode::KeyA,
                        'b' | 'B' => KeyCode::KeyB,
                        'c' | 'C' => KeyCode::KeyC,
                        'd' | 'D' => KeyCode::KeyD,
                        'e' | 'E' => KeyCode::KeyE,
                        'f' | 'F' => KeyCode::KeyF,
                        'g' | 'G' => KeyCode::KeyG,
                        'h' | 'H' => KeyCode::KeyH,
                        'i' | 'I' => KeyCode::KeyI,
                        'j' | 'J' => KeyCode::KeyJ,
                        'k' | 'K' => KeyCode::KeyK,
                        'l' | 'L' => KeyCode::KeyL,
                        'm' | 'M' => KeyCode::KeyM,
                        'n' | 'N' => KeyCode::KeyN,
                        'o' | 'O' => KeyCode::KeyO,
                        'p' | 'P' => KeyCode::KeyP,
                        'q' | 'Q' => KeyCode::KeyQ,
                        'r' | 'R' => KeyCode::KeyR,
                        's' | 'S' => KeyCode::KeyS,
                        't' | 'T' => KeyCode::KeyT,
                        'u' | 'U' => KeyCode::KeyU,
                        'v' | 'V' => KeyCode::KeyV,
                        'w' | 'W' => KeyCode::KeyW,
                        'x' | 'X' => KeyCode::KeyX,
                        'y' | 'Y' => KeyCode::KeyY,
                        'z' | 'Z' => KeyCode::KeyZ,

                        // digits
                        '0' => KeyCode::Digit0,
                        '1' => KeyCode::Digit1,
                        '2' => KeyCode::Digit2,
                        '3' => KeyCode::Digit3,
                        '4' => KeyCode::Digit4,
                        '5' => KeyCode::Digit5,
                        '6' => KeyCode::Digit6,
                        '7' => KeyCode::Digit7,
                        '8' => KeyCode::Digit8,
                        '9' => KeyCode::Digit9,

                        // whitespace
                        ' ' => KeyCode::Space,
                        '\t' => KeyCode::Tab,

                        // common ASCII punctuation
                        '-' => KeyCode::Minus,
                        '=' => KeyCode::Equal,
                        '\\' => KeyCode::Backslash,
                        ';' => KeyCode::Semicolon,
                        ',' => KeyCode::Comma,
                        '.' => KeyCode::Period,
                        '/' => KeyCode::Slash,
                        _ => KeyCode::Unidentified(
                            bevy::input::keyboard::NativeKeyCode::Unidentified,
                        ),
                    })
                } else {
                    None
                }
            }
            Some(Key::Special(ScanCode::ESCAPE)) => Some(KeyCode::Escape),
            Some(Key::Special(ScanCode::UP)) => Some(KeyCode::ArrowUp),
            Some(Key::Special(ScanCode::DOWN)) => Some(KeyCode::ArrowDown),
            Some(Key::Special(ScanCode::LEFT)) => Some(KeyCode::ArrowLeft),
            Some(Key::Special(ScanCode::RIGHT)) => Some(KeyCode::ArrowRight),
            Some(Key::Special(ScanCode::PAGE_UP)) => Some(KeyCode::PageUp),
            Some(Key::Special(ScanCode::PAGE_DOWN)) => Some(KeyCode::PageDown),
            Some(Key::Special(ScanCode::HOME)) => Some(KeyCode::Home),
            Some(Key::Special(ScanCode::END)) => Some(KeyCode::End),
            Some(Key::Special(ScanCode::INSERT)) => Some(KeyCode::Insert),
            Some(Key::Special(ScanCode::DELETE)) => Some(KeyCode::Delete),
            Some(Key::Special(ScanCode::FUNCTION_1)) => Some(KeyCode::F1),
            Some(Key::Special(ScanCode::FUNCTION_2)) => Some(KeyCode::F2),
            Some(Key::Special(ScanCode::FUNCTION_3)) => Some(KeyCode::F3),
            Some(Key::Special(ScanCode::FUNCTION_4)) => Some(KeyCode::F4),
            Some(Key::Special(ScanCode::FUNCTION_5)) => Some(KeyCode::F5),
            Some(Key::Special(ScanCode::FUNCTION_6)) => Some(KeyCode::F6),
            Some(Key::Special(ScanCode::FUNCTION_7)) => Some(KeyCode::F7),
            Some(Key::Special(ScanCode::FUNCTION_8)) => Some(KeyCode::F8),
            Some(Key::Special(ScanCode::FUNCTION_9)) => Some(KeyCode::F9),
            Some(Key::Special(ScanCode::FUNCTION_10)) => Some(KeyCode::F10),
            Some(Key::Special(ScanCode::FUNCTION_11)) => Some(KeyCode::F11),
            Some(Key::Special(ScanCode::FUNCTION_12)) => Some(KeyCode::F12),
            Some(Key::Special(ScanCode::MUTE)) => Some(KeyCode::AudioVolumeMute),
            Some(Key::Special(ScanCode::VOLUME_UP)) => Some(KeyCode::AudioVolumeUp),
            Some(Key::Special(ScanCode::VOLUME_DOWN)) => Some(KeyCode::AudioVolumeDown),
            Some(Key::Special(_)) => None,
            None => None,
        };

        if let Some(code) = code {
            Ok(Some(code))
        } else {
            Ok(None)
        }
    }
}
