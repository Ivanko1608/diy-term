#[derive(Debug)]
pub enum CommandParsingError {
    CommandNotFound(String),
}

impl std::fmt::Display for CommandParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandParsingError::CommandNotFound(str) => write!(f, "{str}: command not found"),
        }
    }
}

// TODO: Implement source?
impl std::error::Error for CommandParsingError {}

pub fn handle_cmd(cmd: &str, args: Vec<&str>) -> Result<(), CommandParsingError> {
    use std::process::exit;

    match cmd {
        "exit" => exit(0),
        "echo" => {
            println!("{}", args.join(" "));
            Ok(())
        }
        _ => Err(CommandParsingError::CommandNotFound(cmd.into())),
    }
}
