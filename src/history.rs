use std::fs::{self, OpenOptions};
use std::io::Write;

const DEFAULT_HISTORY_LENGTH: u32 = 10_000;
const HISTORY_FILE_NAME: &str = "histrory.diyrc";

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

