use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn run(query: &str) -> Result<()> {
    search_dir(".", query)
}

pub fn parse_config(args: &[String]) -> &str {
    let query = if args.len() > 1 {&args[1]} else {""};
    return query;
}

pub fn search_dir(name: &str, query: &str) -> Result<()> {
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
        let _ = search_file(&path, query);
    }
    Ok(())
}

pub fn search_file(name: &str, query: &str) -> Result<()> {
    match open(name) {
        Err(err) => eprintln!("{}: {}", name, err),
        Ok(file) => {
            for (n, line) in file.lines().enumerate() {
                let lin = line?;
                if lin.contains(query) {
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
