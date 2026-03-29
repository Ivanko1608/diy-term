#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    collections::HashMap,
    fs::DirEntry,
    path::PathBuf,
    process::{Command, Stdio, exit},
};

use crate::cmd::{CommandParsingError, fill_bin_cache};

mod builtins;
mod cmd;

fn main() {
    let mut bin_paths = HashMap::<String, PathBuf>::new();

    // TODO: not ignore this maybe?
    let _ = fill_bin_cache(&mut bin_paths);

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

        let result = cmd::handle_builtin_cmd(cmd, args);

        match result {
            Ok(()) => {}
            Err(CommandParsingError::CommandNotFound(cmd)) => {
                let Some(bin_path) = bin_paths.get(&cmd) else {
                    eprintln!("{cmd}: not found");
                    continue;
                };

                println!("{cmd} is {}", bin_path.display())

                // Command::new(bin_path)
                //     .stdout(Stdio::inherit())
                //     .stderr(Stdio::inherit())
                //     .status()
                //     .expect("Command failed!");
            }
            Err(e) => {
                eprintln!("Failed to execute builtin command! {e}");
            }
        }
    }
}
