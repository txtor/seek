use std::env;

fn main() -> seek::SeekResult<()> {
    let args :Vec<String> = env::args().collect();
    let query = seek::Query::from_args(&args);
    seek::run(&query)?;
    Ok(())
}

