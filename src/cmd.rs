use crate::builtins::BuiltinCmd;

use super::builtins;

#[derive(Debug)]
pub enum CommandParsingError {
    CommandNotFound(String),
    TooManyArgs {
        cmd_name: String,
        max_arg_count: usize,
    },
}

impl std::fmt::Display for CommandParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandParsingError::CommandNotFound(str) => write!(f, "{str}: command not found"),
            CommandParsingError::TooManyArgs {
                cmd_name,
                max_arg_count,
            } => write!(f, "{cmd_name}: accepts max {max_arg_count} arguments!"),
        }
    }
}

// TODO: Implement source?
impl std::error::Error for CommandParsingError {}

pub fn parse_builtin_cmd(cmd: &str) -> Result<BuiltinCmd, CommandParsingError> {
    cmd.parse::<BuiltinCmd>()
        .map_err(|_| CommandParsingError::CommandNotFound(cmd.into()))
}

pub fn handle_cmd(cmd: &str, args: Vec<&str>) -> Result<(), CommandParsingError> {
    use super::builtins::BuiltinCmd;
    use std::process::exit;

    match parse_builtin_cmd(cmd)? {
        BuiltinCmd::Exit => exit(0),
        BuiltinCmd::Echo => {
            builtins::echo(&args.join(" "));
            Ok(())
        }
        BuiltinCmd::Type => {
            builtins::type_cmd(&args)?;
            Ok(())
        }
    }
}
