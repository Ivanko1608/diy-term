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
use crate::raw_term::{KbKey, RawMode, get_key};
mod builtins;
mod cmd;
mod file_type;
mod history;
mod raw_term;

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
        print!("$ ");

        // Rust doesn't flush afrer a print! only after println! so the print above won't be flushed
        // unless we force it
        io::stdout().flush().unwrap();

        let mut raw_cmd = String::new();
        {
            let _raw_mod = RawMode::new();
            let cursor = 0;

            loop {
                let mut input_buf = [0u8; 1];
                io::stdin()
                    .read_exact(&mut input_buf)
                    .expect("Failed to read 1 byte from input!");

                match get_key(input_buf[0]) {
                    KbKey::Enter => {
                        print!("\r\n");
                        io::stdout().flush().unwrap();
                        break;
                    }
                    KbKey::CtrlC => {
                        break 'main_loop;
                    }
                    // BACKSPACE
                    KbKey::Backspace => {
                        if raw_cmd.pop().is_some() {
                            // FIXME this only works if you are erasing the start of the cmd, in the
                            //middle you have to now move back inside raw_cmd ALL the chars after the one erased unless raw_cmd is now len(0)

                            // \x08 moves the cursor back one, we then overwrite that with a space
                            // the cursor moves forward once and we move it back again.
                            print!("\x08 \x08");
                            io::stdout().flush().unwrap()
                        }
                    }
                    KbKey::Char(c) => {
                        raw_cmd.push(c);
                        print!("{}", c);
                        io::stdout().flush().unwrap();
                    }
                    KbKey::Left => {
                        print!("\x1b[1D");
                        io::stdout().flush().unwrap();
                    }
                    KbKey::Right => {
                        print!("\x1b[1C");
                        io::stdout().flush().unwrap();
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
