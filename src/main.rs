use std::io::{self, Read, Write};
use std::{
    collections::HashMap,
    path::PathBuf,
    process::{Command, Stdio},
    sync::{LazyLock, RwLock},
};

use std::sync::Mutex;

use crate::cmd::{CommandParsingError, fill_bin_cache};
use crate::history::History;
use crate::raw_term::util::clear_line;
use crate::raw_term::{KbKey, RawMode, get_key};
mod builtins;
mod cmd;
mod file_type;
mod history;
mod raw_term;

macro_rules! print_flush {
    ($($arg:tt)+) => {
        print!($($arg)+);
        io::stdout().flush().expect("Failed to flush Stdout");
    };
}

/// Cache of binary names -> paths, found in all dirs from the PATH env var.
pub static BIN_CACHE: LazyLock<RwLock<HashMap<String, PathBuf>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub static HISTORY: LazyLock<Mutex<History>> = LazyLock::new(|| Mutex::new(History::new()));

fn main() {
    std::panic::set_hook(Box::new(|info| {
        eprintln!("thread panicked: {}", info);
    }));

    // We do this to force lazy lock to initialize,
    // that way the history file will be read before we spawn the thread that truncates it down
    // the line.
    drop(HISTORY.lock().unwrap());

    // TODO: not ignore this maybe?
    let _ = fill_bin_cache();

    std::thread::spawn(|| {
        use std::time::Duration;
        loop {
            std::thread::sleep(Duration::from_secs(15));
            HISTORY.lock().unwrap().write_to_disk();
        }
    });

    'main_loop: loop {
        print_flush!("$ ");

        let mut raw_cmd = String::new();
        {
            let _raw_mod = RawMode::new();
            let mut cursor = raw_cmd.len();

            loop {
                let mut input_buf = [0u8; 1];
                io::stdin()
                    .read_exact(&mut input_buf)
                    .expect("Failed to read 1 byte from input!");

                match get_key(input_buf[0]) {
                    KbKey::Enter => {
                        print_flush!("\r\n");
                        break;
                    }
                    KbKey::CtrlC => {
                        break 'main_loop;
                    }
                    KbKey::Backspace => {
                        if raw_cmd.is_empty() {
                            continue;
                        }

                        if cursor == raw_cmd.len() {
                            raw_cmd.pop();
                        } else {
                            raw_cmd.remove(cursor - 1);
                        }

                        cursor -= 1;

                        clear_line();
                        print_flush!("$ {raw_cmd}");
                    }
                    KbKey::Char(c) => {
                        raw_cmd.push(c);
                        cursor += 1;
                        print_flush!("{}", c);
                    }
                    KbKey::Left => {
                        if cursor != 0 {
                            cursor -= 1;
                        } else {
                            continue;
                        }

                        print_flush!("\x1b[1D");
                    }
                    KbKey::Right => {
                        cursor += 1;
                        print_flush!("\x1b[1C");
                    }
                    KbKey::Unknown(b) => {
                        eprint!("Some other byte: {:?} \r\n", char::from(b));
                        io::stderr().flush().unwrap();
                    }
                    KbKey::UnknownMultiByte(v) => {
                        eprint!("{:?}", v);
                        io::stderr().flush().unwrap();
                    }
                    key => {
                        eprint!("Handling key: {:?} is not yet implemented! \r\n$ ", key);
                        io::stderr().flush().unwrap();
                    }
                }
            }
        }

        if raw_cmd.trim().is_empty() {
            continue;
        }

        let mut cmd_parts = raw_cmd.split_whitespace();

        let cmd = cmd_parts
            .next()
            .expect("Could not get the main comand part");
        let args: Vec<&str> = cmd_parts.collect();

        let result = cmd::handle_builtin_cmd(cmd, &args);

        match result {
            Ok(()) => {
                HISTORY.lock().unwrap().add_cmd(cmd, args);
            }
            Err(CommandParsingError::CommandNotFound(cmd)) => {
                let Some(_bin_path) = BIN_CACHE
                    .read()
                    .expect("Failed to lock BIN_CACHE for reading!")
                    .get(&cmd)
                else {
                    eprintln!("{cmd}: not found");
                    continue;
                };

                // I disagree with this and think we should call full path. but fine
                Command::new(&cmd)
                    .args(&args)
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .status()
                    .expect("Command failed!");

                HISTORY.lock().unwrap().add_cmd(&cmd, args);
            }
            Err(e) => {
                eprintln!("Failed to execute builtin command! {e}");
            }
        }
    }
}
