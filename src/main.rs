#[allow(unused_imports)]
use std::io::{self, Write};

mod cmd;

fn main() {
    // TODO: Uncomment the code below to pass the first stage
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

        let _ = cmd::handle_cmd(cmd, args).map_err(|err| {
            eprintln!("{}", err);
            err
        });
    }
}
