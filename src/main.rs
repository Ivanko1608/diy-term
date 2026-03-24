use core::fmt;
#[allow(unused_imports)]
use std::io::{self, Write};

#[derive(Debug)]
enum CommandParsingError {
    CommandNotFound(String),
}

impl fmt::Display for CommandParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandParsingError::CommandNotFound(str) => write!(f, "{str}: command not found"),
        }
    }
}

// TODO Implement source?
impl std::error::Error for CommandParsingError {}

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

        let cmd = raw_cmd.trim();

        let _ = handle_cmd(cmd).map_err(|err| {
            eprintln!("{}", err);
            err
        });
    }
}

fn handle_cmd(cmd: &str) -> Result<(), CommandParsingError> {
    use std::process::exit;

    match cmd {
        "exit" => exit(0),
        "echo" => {
            println!("{}", cmd);
            Ok(())
        }
        _ => Err(CommandParsingError::CommandNotFound(cmd.into())),
    }
}
