use std::fs;
use std::fs::{ReadDir,DirEntry};

pub struct DirSearcher {
    recur :bool,
    entries :ReadDir,
    dirs :Vec<String>
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
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut item = self.entries.next();
            while item.is_none() {
                if self.dirs.is_empty() {
                    //println!("none");
                    return None;
                }
                let dir :String = self.dirs.remove(0);
                //println!("pop {dir}");
                self.entries = fs::read_dir(dir).unwrap();
                item = self.entries.next();
            }
            //println!("item {item:?}");
            let entry :DirEntry = item?.unwrap();
            let path :String = entry
                            .path()
                            .to_string_lossy()
                            .into_owned();
            //println!("path: {path}");
            if entry.file_type().unwrap().is_dir() {
                if self.recur { self.dirs.push(path); }
            } else {
                return Some(path)
            }
        }
    }
}