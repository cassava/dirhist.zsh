use clap::Parser;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use itertools::Itertools;
use std::collections::HashSet;

pub const DEFAULT_DIRHIST_SIZE: usize = 10000;
pub const DEFAULT_DIRHIST_FILE: &'static str = ".directory_history";
pub const DIRHIST_FILE_ENV: &'static str = "ZSH_DIRHIST_FILE";

pub struct HistoryFile {
    path: PathBuf,
    buffer: Vec<u8>,
}

impl HistoryFile {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            buffer: Vec::new(),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn read_all(&mut self) -> impl Iterator<Item = HistoryEntry> + '_ {
        if self.buffer.is_empty() {
            let mut file = std::fs::File::open(&self.path).unwrap();
            file.read_to_end(&mut self.buffer).unwrap();
        }

        self.buffer
            .split(|b| *b == 0)
            .map(|b| String::from_utf8_lossy(b).into_owned())
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.parse::<HistoryEntry>().unwrap())
    }

    pub fn read_all_by_dir(
        &mut self,
        dir: impl Into<String>,
    ) -> impl Iterator<Item = HistoryEntry> + '_ {
        let dir = dir.into();
        let (dir_entries, other_entries): (Vec<_>, Vec<_>) =
            self.read_all().partition(|e| e.directory == dir);
        dir_entries.into_iter().chain(other_entries.into_iter())
    }

    pub fn read_all_with(&mut self, dir: impl Into<String>, substring: impl Into<String>) -> Box<dyn Iterator<Item = HistoryEntry> + '_> {
        let (dir, substring) = (dir.into(), substring.into());
        let entries: Box<dyn Iterator<Item = HistoryEntry>> = if dir.is_empty() {
            Box::new(self.read_all().filter(move |e| e.command.starts_with(&substring)))
        } else {
            Box::new(self.read_all_by_dir(dir).filter(move |e| e.command.starts_with(&substring)))
        };
        entries
    }

    pub fn read_commands_with(&mut self, dir: impl Into<String>, substring: impl Into<String>) -> Box<dyn Iterator<Item = String> + '_> {
        let mut set = HashSet::new();
        Box::new(self.read_all_with(dir, substring).filter_map(
            move |e| {
                if set.contains(&e.command) {
                    None
                } else {
                    set.insert(e.command.clone());
                    Some(e.command)
                }
            }
        ))
    }
}

impl Default for HistoryFile {
    fn default() -> Self {
        let dirhist_file = 
            std::env::var(DIRHIST_FILE_ENV)
                .map(|f| PathBuf::from(f))
                .or_else(|_| std::env::var("HOME")
                    .map(|home| PathBuf::from(home).join(DEFAULT_DIRHIST_FILE))
                );

        match dirhist_file {
            Ok(path) => HistoryFile::new(path),
            Err(_) => panic!("require either ZSH_DIRHIST_FILE or HOME environment variable"),
        }
    }
}

pub struct HistoryEntry {
    /// Time the command was run in epoch seconds.
    pub timestamp: u64,
    pub duration: u64,
    pub directory: String,
    pub command: String,
}

impl FromStr for HistoryEntry {
    // TODO: Implement error handling
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.splitn(3, ";");
        let (ts, dur) = {
            let s = iter.next().unwrap();
            let items: Vec<_> = s.split(":").collect();
            assert!(items.len() <= 3);
            let ts = items[1].trim().parse().expect("cannot parse timestamp");
            let dur = if items.len() == 3 {
                items[2].trim().parse().expect("cannot parse duration")
            } else {
                0
            };
            (ts, dur)
        };
        Ok(Self {
            timestamp: ts,
            duration: dur,
            directory: iter.next().unwrap().into(),
            command: iter.next().unwrap().into(),
        })
    }
}

/// Directory history retriever
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory to fetch history for
    #[arg(short, long, default_value_t = String::new())]
    directory: String,

    /// Return list of indices which match a given substring
    #[arg(short, long, default_value_t = String::new())]
    substring: String,

    /// Return only indices
    #[arg(short, long)]
    indices: bool,
}

fn main() {
    let args = Args::parse();
    let mut history = HistoryFile::default();

    let entries = history.read_commands_with(args.directory, args.substring);
    if args.indices {
        let mut n = 0;
        for (i, _) in entries.enumerate() {
            println!("{}", i);
            n += 1;
        }
        if n == 0 {
            println!("NONE");
        }
    } else {
        let list: String = entries.intersperse("\0\n".into()).collect();
        println!("{}", list);
    }
}
