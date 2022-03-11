use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct FileSearcher<'a> {
    file :Box<dyn BufRead>,
    query: &'a crate::Query,
    filename : &'a str,
    line_number : u32,
}

pub struct FileMatch<'a> {
    pub filename : &'a str,
    pub line_number : u32,
    pub line : String
}
impl<'a> PartialEq for FileMatch<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.filename == other.filename
        && self.line_number == other.line_number
        && self.line == other.line
    }
}
impl<'a> Eq for FileMatch<'a> {}

impl<'a> std::fmt::Display for FileMatch<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}:{}",self.filename,self.line_number,self.line)
    }
}

impl<'a> FileSearcher<'a> {
    pub fn new(filename: &'a str, query: &'a crate::Query) -> crate::SeekResult<Self> {
        match open(filename) {
            Err(e) => Err(e),
            Ok(file) => Ok(FileSearcher { 
                file: file,
                query: query,
                filename: filename,
                line_number: 0
            })
        }
    }
}

impl<'a> Iterator for FileSearcher<'a> {
    type Item = crate::SeekResult<FileMatch<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut lin = String::new();
        loop {
            match self.file.read_line(&mut lin) {
                Err(e) => return Some(Err(Box::new(e))),
                Ok(0) => return None,
                Ok(_) => {
                    self.line_number += 1;
                    if lin.chars().last() == Some('\n') { _ = lin.pop(); }
                    let mut found :bool = true;
                    for target in &self.query.targets {
                        if !lin.contains(target) {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        return Some(Ok(FileMatch { 
                            filename: self.filename,
                            line_number: self.line_number,
                            line :lin
                        }));
                    };
                    lin.clear();
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
