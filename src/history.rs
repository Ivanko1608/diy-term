use std::collections::VecDeque;
use std::fs::{self, File};
use std::io::Write;
use std::io::{BufRead, BufReader, BufWriter};
use std::path::PathBuf;

const DEFAULT_HISTORY_LENGTH: usize = 10_000;
const HISTORY_FILE_NAME: &str = ".diyhistory";

pub struct History {
    path: PathBuf,
    history: VecDeque<String>,
}

// TODO: Implement an OnExit callback to dump history file to fs.
impl History {
    pub fn new() -> History {
        let home_dir = std::env::home_dir().expect("Failed to get home dir!");

        let path = home_dir.join(HISTORY_FILE_NAME);

        let history = History::load_from_fs();

        History { path, history }
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

    pub fn get_history(&self) -> &VecDeque<String> {
        &self.history
    }

    pub fn add_cmd(&mut self, cmd: &str, args: Vec<&str>) {
        if self.history.len() + 1 > DEFAULT_HISTORY_LENGTH {
            self.history.pop_front();
        }

        self.history.push_back(format!("{cmd} {}", args.join(" ")));
    }

    pub fn write_to_disk(&mut self) {
        let file = fs::File::options()
            .truncate(true)
            .create(true)
            .write(true)
            .open(&self.path)
            .expect("Failed to open history file");

        let mut writer = BufWriter::new(file);

        for cmd in self.history.iter() {
            let _ = writeln!(writer, "{cmd}").map_err(|e| {
                eprintln!("Failed to write to history file! {e}");
            });
        }
        writer.flush().expect("Failed to flush writer!");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_history_truncates_before_passing_limit() {
        let mut history = History::new();
        for _ in 0..10_005 {
            history.add_cmd("rm", vec!["-rf", "./coffee"]);
        }
        assert_eq!(history.get_history().len(), DEFAULT_HISTORY_LENGTH);
    }
}
