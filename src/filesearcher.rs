use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct FileSearcher<'a> {
    file :Box<dyn BufRead>,
    query: &'a crate::Query,
    filename : String,
    line_number : u32,
}

pub struct FileMatch {
    pub filename : String,
    pub line_number : u32,
    pub line : String
}

impl<'a> FileSearcher<'a> {
    pub fn new(filename: &str, query: &'a crate::Query) -> crate::SeekResult<Self> {
        match open(filename) {
            Err(e) => Err(e),
            Ok(file) => Ok(FileSearcher { 
                file: file,
                query: query,
                filename: String::from(filename),
                line_number: 0
            })
        }
    }
}

impl<'a> Iterator for FileSearcher<'a> {
    type Item = crate::SeekResult<FileMatch>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut lin = String::new();
        match self.file.read_line(&mut lin) {
            Err(e) => Some(Err(Box::new(e))),
            Ok(0) => None,
            Ok(_) => {
                lin = lin.trim().to_string(); //how to avoid allocation?
                let mut found :bool = true;
                for target in &self.query.targets {
                    if !lin.contains(target) {
                        found = false;
                        break;
                    }
                }
                if found {
                    Some(Ok(FileMatch { 
                        filename: self.filename.clone(),
                        line_number: self.line_number,
                        line :lin
                    }))
                } else {
                    self.next()
                }
            }
        }
    }
}

fn open(filename: &str) -> crate::SeekResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
