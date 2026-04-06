#![cfg(unix)]

use libc::{STDIN_FILENO, TCSANOW, cfmakeraw, tcgetattr, tcsetattr, termios};
use std::io::{self, Error, Read};

// Control sequence introducer (ANSI)
const CSI: u8 = b'\x9b';

//ANSI: Escape sequence
const ESC: u8 = b'\x1b';

#[derive(Debug)]
pub enum KbKey {
    Char(char),
    Up,
    Down,
    Left,
    Right,
    Backspace,
    Enter,
    CtrlC,
    Unknown(u8),
    UnknownMultiByte(Vec<u8>),
}

pub struct RawMode {
    original_termios: termios,
}

impl RawMode {
    pub fn new() -> Self {
        let mut termios: termios = unsafe { std::mem::zeroed() };

        let ret = unsafe { tcgetattr(STDIN_FILENO, &mut termios) };

        if ret == -1 {
            panic!("tcgetattr failed: {}", Error::last_os_error());
        }

        let raw_mode = RawMode {
            original_termios: termios.clone(),
        };

        unsafe { cfmakeraw(&mut termios) };

        let ret = unsafe { tcsetattr(STDIN_FILENO, TCSANOW, &termios) };

        if ret == -1 {
            panic!("tcsetattr failed: {}", Error::last_os_error());
        }
        raw_mode
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        let ret = unsafe { tcsetattr(STDIN_FILENO, TCSANOW, &self.original_termios) };

        if ret == -1 {
            panic!("tcsetattr failed: {}", Error::last_os_error());
        }
    }
}

pub fn get_key(raw_key: u8) -> KbKey {
    match raw_key {
        // ENTER
        b'\r' | b'\n' => KbKey::Enter,

        // CTRL + C
        b'\x03' => KbKey::CtrlC,

        // BACKSPACE
        b'\x7f' | b'\x08' => KbKey::Backspace,

        b if b.is_ascii_graphic() || b == b' ' => KbKey::Char(char::from(b)),

        //ANSI Escape parsing
        ESC => {
            let mut buf = [0u8, 2];

            io::stdin()
                .read_exact(&mut buf)
                .expect("Failed to read 2 bytes from input!");

            if !buf[0] == CSI {
                return KbKey::UnknownMultiByte(std::iter::once(ESC).chain(buf).collect());
            }

            match char::from(buf[1]) {
                'A' => KbKey::Up,
                'B' => KbKey::Down,
                'C' => KbKey::Right,
                'D' => KbKey::Left,
                _ => KbKey::UnknownMultiByte(std::iter::once(ESC).chain(buf).collect()),
            }
        }
        b => KbKey::Unknown(b),
    }
}
