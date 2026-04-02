use std::fs::{self, File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

const DEFAULT_HISTORY_LENGTH: u32 = 10_000;
const HISTORY_FILE_NAME: &str = "histrory.diyrc";
static HISTORY_FILE_HANDLE: OnceLock<Mutex<History>> = OnceLock::new();

struct History {
    writer: BufWriter<File>,
    line_count: usize,
    path: PathBuf,
    history: Vec<String>,
}

// TODO: Implement an OnExit callback to dump history file to fs.
impl History {
    pub fn new() -> History {
        let home_dir = std::env::home_dir().expect("Failed to get home dir!");

        let path = home_dir.join(HISTORY_FILE_NAME);

        let file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .open(&path)
            .expect("Failed to open history file!");

        History {
            writer: BufWriter::new(file),
            line_count: 0, // TODO: Tempoprary placeholder
            path,
            history: vec![], // TODO: Temproary placeholder.
        }
    }

    fn load_from_fs() -> Option<Vec<String>> {
        let home_dir = std::env::home_dir().expect("Failed to get home dir!");

        let path = home_dir.join(HISTORY_FILE_NAME);

        if !path.is_file() {
            return None;
        }

        todo!()
    }
}

pub fn get_history() {
    let home_dir = std::env::home_dir().expect("Failed to get home dir!");
    let path = home_dir.join(HISTORY_FILE_NAME);

    let history = fs::read_to_string(path).expect("Failed to read {HISTORY_FILE_NAME} file!");
    println!("{history}")
}

// TODO: Delete oldest lines once we are over HISTORY_LENGTH
pub fn add_cmd_to_history(cmd: &str, args: Vec<&str>) {
    let home_dir = std::env::home_dir().expect("Failed to get home dir!");

    let path = home_dir.join(HISTORY_FILE_NAME);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .expect("Failed to open history file!");

    writeln!(file, "{cmd} {}", args.join(" ")).expect("Failed to write command to history file!");
}
