use std::str::FromStr;

use crate::{BIN_CACHE, cmd::CommandParsingError};

pub enum BuiltinCmd {
    Echo,
    Type,
    Exit,
    PrintWorkingDirectory,
    ChangeDirectory,
}

impl FromStr for BuiltinCmd {
    type Err = CommandParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "type" => Ok(BuiltinCmd::Type),
            "echo" => Ok(BuiltinCmd::Echo),
            "exit" => Ok(BuiltinCmd::Exit),
            "pwd" => Ok(BuiltinCmd::PrintWorkingDirectory),
            "cd" => Ok(BuiltinCmd::ChangeDirectory),
            _ => Err(CommandParsingError::CommandNotFound(s.into())),
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

    if args[0].parse::<BuiltinCmd>().is_ok() {
        println!("{} is a shell builtin", args[0]);

        return Ok(());
    }

    let bin_cache_guard = BIN_CACHE.read().expect("Failed to read from BIN_CACHE");

    if let Some(bin) = bin_cache_guard.get(args[0]) {
        println!(
            "{} is {}",
            args[0],
            bin.to_str().expect("Failed to convers PathBuf to &str")
        );
        return Ok(());
    }
    Err(CommandParsingError::CommandNotFound(args[0].into()))
}

pub fn echo(args: &str) {
    println!("{}", args);
}
