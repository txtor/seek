use std::fs;
use std::fs::{ReadDir,DirEntry};

pub struct DirSearcher {
    recur :bool,
    entries :ReadDir,
    dirs :Vec<DirEntry>
}

impl DirSearcher {
    pub fn new(dir: &str, recur: bool) -> crate::SeekResult<Self> {
        Ok(DirSearcher { 
            recur: recur,
            entries: fs::read_dir(dir)?,
            dirs: Vec::new()
        })
    }
}

impl Iterator for DirSearcher {
    type Item = DirEntry;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut item = self.entries.next();
            while item.is_none() {
                if self.dirs.is_empty() {
                    return None;
                }
                let dir :DirEntry = self.dirs.remove(0);
                self.entries = fs::read_dir(dir.path()).unwrap();
                item = self.entries.next();
            }
            let entry :DirEntry = item?.unwrap();
            if entry.file_type().unwrap().is_dir() {
                if self.recur { self.dirs.push(entry); }
            } else {
                return Some(entry)
            }
        }
    }
}