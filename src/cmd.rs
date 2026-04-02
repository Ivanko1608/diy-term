use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::{env, io};

use crate::BIN_CACHE;
use crate::history::History;

use super::builtins;
use super::file_type;

const PATH_ENV_KEY: &str = "PATH";
static HISTORY_FILE_HANDLE: OnceLock<Mutex<History>> = OnceLock::new();

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
        BuiltinCmd::PrintWorkingDirectory => {
            println!(
                "{}",
                env::current_dir()
                    .expect("Failed to get current directory.")
                    .display()
            );
            Ok(())
        }
        BuiltinCmd::History => {
            get_history();
            Ok(())
        }
        BuiltinCmd::ChangeDirectory => {
            if let Some(path) = args.first() {
                let mut path = PathBuf::from(path);

                if path == Path::new("~") {
                    let home_path = std::env::home_dir().expect("Failed to get home dir!");

                    path = home_path;
                }

                let path_exists = path.try_exists().expect("Failed to check is path exists!");

                if !path_exists {
                    println!("cd: {}: No such file or directory", path.display());
                    return Ok(());
                }

                env::set_current_dir(path).expect("Failed to set current dir!");
                Ok(())
            } else {
                eprintln!("cd takes exactly 1 argument! cd <your-dir>");
                Ok(())
            }
        }
    }
}

// TODO: Invalidate cache when PATH changes.
// FIXME: Get rid of pub
pub fn fill_bin_cache() -> Result<(), io::Error> {
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

            BIN_CACHE
                .write()
                .expect("Failed to lock BIN_CACHE")
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
