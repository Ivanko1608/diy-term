#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    collections::HashMap,
    path::PathBuf,
    process::{Command, Stdio},
    sync::{LazyLock, RwLock},
};

use crate::{
    cmd::{CommandParsingError, fill_bin_cache},
    history::add_cmd_to_history,
};

mod builtins;
mod cmd;
mod file_type;
mod history;

/// Cache of binary names -> paths, found in all dirs from the PATH env var.
pub static BIN_CACHE: LazyLock<RwLock<HashMap<String, PathBuf>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

fn main() {
    // TODO: not ignore this maybe?
    let _ = fill_bin_cache();

    loop {
        print!("$ ");

        // Rust doesn't flush afrer a print! only after println! so the print above won't be flushed
        // unless we force it
        io::stdout().flush().unwrap();

        let mut raw_cmd = String::new();

        io::stdin()
            .read_line(&mut raw_cmd)
            .expect("Failed to read user input");

        let mut cmd_parts = raw_cmd.split_whitespace();

        let cmd = cmd_parts
            .next()
            .expect("Could not get the main comand part");
        let args: Vec<&str> = cmd_parts.collect();

        let result = cmd::handle_builtin_cmd(cmd, &args);

        match result {
            Ok(()) => {
                add_cmd_to_history(cmd, args);
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

                add_cmd_to_history(&cmd, args);
            }
            Err(e) => {
                eprintln!("Failed to execute builtin command! {e}");
            }
        }
    }
}
