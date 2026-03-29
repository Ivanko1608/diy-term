use crate::builtins::BuiltinCmd;
use std::{collections::HashMap, env, io, path::PathBuf};

use super::builtins;
use super::file_type;

const PATH_ENV_KEY: &str = "PATH";

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
            CommandParsingError::CommandNotFound(str) => write!(f, "{str}: not found"),
            CommandParsingError::TooManyArgs {
                cmd_name,
                max_arg_count,
            } => write!(f, "{cmd_name}: accepts max {max_arg_count} arguments!"),
        }
    }
}

// TODO: Implement source?
impl std::error::Error for CommandParsingError {}

pub fn handle_builtin_cmd(cmd: &str, args: &Vec<&str>) -> Result<(), CommandParsingError> {
    use super::builtins::BuiltinCmd;
    use std::process::exit;

    match cmd.parse::<BuiltinCmd>()? {
        BuiltinCmd::Exit => exit(0),
        BuiltinCmd::Echo => {
            builtins::echo(&args.join(" "));
            Ok(())
        }
        BuiltinCmd::Type => {
            builtins::type_cmd(args)?;
            Ok(())
        }
    }
}

// TODO: Invalidate cache when PATH changes.
// FIXME: Get rid of pub
pub fn fill_bin_cache(bin_cache: &mut HashMap<String, PathBuf>) -> Result<(), io::Error> {
    let paths = env::var_os(PATH_ENV_KEY).expect("No PATH env var found!");

    for path in env::split_paths(&paths) {
        if !path.is_dir() {
            continue;
        };

        let dir_contents = path.read_dir().expect("Failed to read dir!");

        for dir_entry in dir_contents.filter_map(|res| {
            res.inspect_err(|e| eprintln!("Failed to get dir entry {e}"))
                .ok()
        }) {
            if !dir_entry.file_type()?.is_file()
                || !file_type::is_executable(&dir_entry.path()).unwrap_or_else(|e| {
                    eprintln!("Failed to get file type: {e} ");
                    false
                })
            {
                continue;
            }

            bin_cache
                .entry(
                    dir_entry
                        .file_name()
                        .into_string()
                        .expect("failed to convert os string"),
                )
                .or_insert(dir_entry.path());
        }
    }

    Ok(())
}
