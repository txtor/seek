pub mod dirsearcher;
pub mod filesearcher;

pub type SeekResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Query {
    targets :Vec<String>
}

impl Query {
    pub fn new(targets :Vec<String>) -> Self {
        Query { targets }
    }
    pub fn from_strs(targets :&[&str]) -> Self {
        Query::new(targets
            .into_iter()
            .map(|f| String::from(*f))
            .collect()
        )
    }
    pub fn get_targets(&self) -> &[String] {
        &self.targets
    }
}

pub fn run(query: &Query) -> SeekResult<()> {
    let dsearcher = dirsearcher::DirSearcher::new(".", true)?;
    for dir_entry in dsearcher {
        if dir_entry.path().is_dir() { continue; }
        let path :String = dir_entry
            .path()
            .to_string_lossy()
            .into_owned();
        match filesearcher::FileSearcher::new(&path, query) {
            Err(e) => eprintln!("{}: {}", path, e),
            Ok(fsearcher) => {
                for linr in fsearcher {
                    match linr {
                        Err(e) => {
                            if !ignore_error(&e) { eprintln!("{}: {}", path, e); }
                            break;
                        },
                        Ok(m) => println!("{m}")
                    }
                }
            }
        };
    };
    Ok(())
}

fn ignore_error(e :&Box<dyn std::error::Error>) -> bool {
    match e.downcast_ref::<std::io::Error>() {
        Some(ee) if ee.kind() == std::io::ErrorKind::InvalidData =>
            true,
        _ => false,
    }
}

pub fn parse_query(args: &[String]) -> Query {
    let targets :Vec<String> = if args.len() > 1 {
        args[1..].to_vec()
    } else {
        Vec::new()
    };
    Query { targets }
}