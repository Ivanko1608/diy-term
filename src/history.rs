use std::collections::VecDeque;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter};
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

const DEFAULT_HISTORY_LENGTH: u32 = 10_000;
const HISTORY_FILE_NAME: &str = "histrory.diyrc";

pub struct History {
    writer: BufWriter<File>,
    line_count: usize,
    path: PathBuf,
    history: VecDeque<String>,
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

        let history = History::load_from_fs();

        History {
            writer: BufWriter::new(file),
            line_count: history.len(),
            path,
            history,
        }
    }

    fn load_from_fs() -> VecDeque<String> {
        // NOTE: Maybe not crash because of this?
        let home_dir = std::env::home_dir().expect("Failed to get home dir!");

        let path = home_dir.join(HISTORY_FILE_NAME);

        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to open history file! {e}");
                return VecDeque::default();
            }
        };

        let file = BufReader::new(file);

        file.lines().map_while(Result::ok).collect()
    }

    pub fn get_history(&self) {
        self.history.iter().for_each(|item| print!("{item}"));
    }

    pub fn add_cmd(&mut self, cmd: &str, args: Vec<&str>) {
        // TODO: Check history size.

        self.history.push_back(format!("{cmd} {}", args.join(" ")));
    }
}

// pub fn get_history() {
//     let home_dir = std::env::home_dir().expect("Failed to get home dir!");
//     let path = home_dir.join(HISTORY_FILE_NAME);
//
//     let history = fs::read_to_string(path).expect("Failed to read {HISTORY_FILE_NAME} file!");
//     println!("{history}")
// }
//
// pub fn add_cmd_to_history(cmd: &str, args: Vec<&str>) {
//     let home_dir = std::env::home_dir().expect("Failed to get home dir!");
//
//     let path = home_dir.join(HISTORY_FILE_NAME);
//
//     let mut file = OpenOptions::new()
//         .append(true)
//         .create(true)
//         .open(path)
//         .expect("Failed to open history file!");
//
//     writeln!(file, "{cmd} {}", args.join(" ")).expect("Failed to write command to history file!");
// }
