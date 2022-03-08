use std::fs::File;
use std::io::{self, BufRead, BufReader};
pub mod dirsearcher;

pub type SeekResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Query {
    targets :Vec<String>
}

impl Query {
    pub fn get_targets(&self) -> &[String] {
        &self.targets
    }
}

pub fn run(query: &Query) -> SeekResult<()> {
    let searcher = dirsearcher::DirSearcher::new(".", true)?;
    for dir_entry in searcher {
        if dir_entry.path().is_dir() { continue; }
        let path :String = dir_entry
            .path()
            .to_string_lossy()
            .into_owned();
        search_file(&path, query)?;
    }
    Ok(())
}

pub fn parse_query(args: &[String]) -> Query {
    let targets :Vec<String> = if args.len() > 1 {
        args[1..].to_vec()
    } else {
        Vec::new()
    };
    Query { targets }
}

pub fn search_file(name: &str, query: &Query) -> SeekResult<()> {
    match open(name) {
        Err(err) => eprintln!("{}: {}", name, err),
        Ok(file) => {
            for (n, line) in file.lines().enumerate() {
                if let Err(e) = line {
                    if e.kind() != std::io::ErrorKind::InvalidData {
                        eprintln!("{}: {}", name, e);
                    }
                    break;
                }
                let lin = line?;
                let mut found :bool = true;
                for target in &query.targets {
                    if !lin.contains(target) {
                        found = false;
                        break;
                    }
                }
                if found {
                    println!("{name}:{n}:{lin}");
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> SeekResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
