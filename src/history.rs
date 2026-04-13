use std::collections::VecDeque;
use std::fs::{self, File};
use std::io::{self, Write};
use std::io::{BufRead, BufReader, BufWriter};
use std::path::{Path, PathBuf};

const DEFAULT_HISTORY_LENGTH: usize = 10_000;
const HISTORY_FILE_NAME: &str = ".diyhistory";

pub struct History {
    path: PathBuf,
    history: VecDeque<String>,
    index: Option<usize>,
}

// TODO: Implement an OnExit callback to dump history file to fs.
impl History {
    pub fn new() -> History {
        let home_dir = std::env::home_dir().expect("Failed to get home dir!");

        let path = home_dir.join(HISTORY_FILE_NAME);

        let history = History::load_from_fs(&path);

        History {
            path,
            history,
            index: None,
        }
    }

    pub fn with_path(path: PathBuf) -> History {
        let history = History::load_from_fs(&path);
        History {
            path,
            history,
            index: None,
        }
    }

    fn load_from_fs(path: &Path) -> VecDeque<String> {
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
        if self.history.len() >= DEFAULT_HISTORY_LENGTH {
            self.history.pop_front();
        }

        self.history.push_back(format!("{cmd} {}", args.join(" ")));
    }

    pub fn prev(&mut self) -> Option<&str> {
        let index = self.index?;

        if let Some(cmd) = self.history.get(index.checked_add(1)?) {
            self.index = Some(index + 1);
            Some(cmd)
        } else {
            None
        }
    }

    pub fn next(&mut self) -> Option<&str> {
        let index = self.index.unwrap_or(self.history.len());

        // If index is at 0 checked_sub will return None and ? returns it immediately.
        // otherwise we get the next command and return it if there is something to return.
        if let Some(cmd) = self.history.get(index.checked_sub(1)?) {
            self.index = Some(index - 1);
            Some(cmd)
        } else {
            None
        }
    }

    pub fn reset_browsing(&mut self) {
        self.index = None;
    }

    pub fn write_to_disk(&mut self) -> Result<(), io::Error> {
        let file = fs::File::options()
            .truncate(true)
            .create(true)
            .write(true)
            .open(&self.path)?;

        let mut writer = BufWriter::new(file);

        for cmd in self.history.iter() {
            writeln!(writer, "{cmd}")?;
        }
        writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::{time::UNIX_EPOCH, vec};

    use super::*;

    fn get_random_history_path() -> PathBuf {
        PathBuf::from(format!(
            "/tmp/{}.diytest",
            std::time::SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    }

    #[test]
    fn test_history_caps_at_limit() {
        let mut history = History::with_path(get_random_history_path());
        for _ in 0..10_005 {
            history.add_cmd("rm", vec!["-rf", "./coffee"]);
        }
        assert_eq!(history.get_history().len(), DEFAULT_HISTORY_LENGTH);
    }

    //TODO: Fix this horrid monstruosity.
    #[test]
    fn test_history_next_and_prev_random() {
        let mut history = History::with_path(get_random_history_path());
        history.add_cmd("foo", vec!["bar"]);
        history.add_cmd("bar", vec!["baz"]);
        history.add_cmd("lel", vec!["osdoods"]);

        let el = history.next().unwrap();

        assert_eq!(el, "lel osdoods");

        let el = history.next().unwrap();

        assert_eq!(el, "bar baz");

        let el = history.next().unwrap();

        assert_eq!(el, "foo bar");

        let el = history.prev().unwrap();
        assert_eq!(el, "bar baz");

        let el = history.next().unwrap();

        assert_eq!(el, "foo bar");

        assert!(history.next().is_none());

        let el = history.prev().unwrap();
        assert_eq!(el, "bar baz");

        let el = history.prev().unwrap();

        assert_eq!(el, "lel osdoods");

        assert!(history.prev().is_none());
    }
}
