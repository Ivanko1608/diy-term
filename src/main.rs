#[allow(unused_imports)]
use std::io::{self, Write};
use std::{collections::HashMap, fs::DirEntry, path::PathBuf};

use crate::cmd::fill_bin_cache;

mod builtins;
mod cmd;

// fn main() {
//     let mut bin_paths = HashMap::<String, PathBuf>::new();
//
//     fill_bin_cache(&mut bin_paths);
//
//     println!("{:?}", bin_paths)
// }
//
fn main() {
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

        let _ = cmd::handle_builtin_cmd(cmd, args).map_err(|err| {
            eprintln!("{}", err);
            err
        });
    }
}
