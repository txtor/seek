use std::fs;
use std::fs::{ReadDir,DirEntry};
use std::path::PathBuf;

pub struct DirSearcher {
    recur :bool,
    entries :ReadDir,
    dirs :Vec<PathBuf>
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
    type Item = crate::SeekResult<DirEntry>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut item = self.entries.next();
        while item.is_none() && !self.dirs.is_empty() {
            let dir :PathBuf = self.dirs.remove(0);
            match fs::read_dir(dir) {
                Ok(entries) => self.entries = entries,
                Err(e) => return Some(Err(Box::new(e)))
            }
            item = self.entries.next();
        }
        match item {
            None => None,
            Some(Ok(entry)) => {
                if self.recur { 
                    if let Ok(file_type) = entry.file_type() {    
                        if file_type.is_dir() {                    
                            self.dirs.push(entry.path().clone()); 
                        }
                    }
                }
                Some(Ok(entry))
            },
            Some(Err(e)) => Some(Err(Box::new(e)))
        }
    }
}