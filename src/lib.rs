use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Query {
    targets :Vec<String>
}

impl Query {
    pub fn get_targets(&self) -> &[String] {
        &self.targets
    }
}

pub fn run(query: &Query) -> Result<()> {
    search_dir(".", query)
}

pub fn parse_config(args: &[String]) -> Query {
    let targets :Vec<String> = if args.len() > 1 {
        args[1..].to_vec()
    } else {
        Vec::new()
    };
    Query { targets }
}

pub fn search_dir(name: &str, query: &Query) -> Result<()> {
    let files = fs::read_dir(name)?;
    for file in files {
        let entry = file?;
        if entry.file_type().unwrap().is_dir() { 
            continue;
        };
        let path :String = entry
                        .path()
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .into_owned();
        let _ = search_file(&path, &query);
    }
    Ok(())
}

pub fn search_file(name: &str, query: &Query) -> Result<()> {
    match open(name) {
        Err(err) => eprintln!("{}: {}", name, err),
        Ok(file) => {
            for (n, line) in file.lines().enumerate() {
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

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
