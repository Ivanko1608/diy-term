use std::str::FromStr;

use crate::cmd::{self, CommandParsingError};

pub enum BuiltinCmd {
    Echo,
    Type,
    Exit,
}

impl FromStr for BuiltinCmd {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "type" => Ok(BuiltinCmd::Type),
            "echo" => Ok(BuiltinCmd::Echo),
            "exit" => Ok(BuiltinCmd::Exit),
            _ => Err(format!("Command: {s} not found in BuiltinCmd enum!")),
        }
    }
}

pub fn type_cmd(args: &[&str]) -> Result<(), CommandParsingError> {
    if args.len() > 1 {
        return Err(CommandParsingError::TooManyArgs {
            cmd_name: "type".into(),
            max_arg_count: 1,
        });
    }

    let _ = cmd::parse_builtin_cmd(args[0])?;

    println!("{} is a shell builtin", args[0]);

    Ok(())
}

pub fn echo(args: &str) {
    println!("{}", args);
}
