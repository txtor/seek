use std::env;

fn main() -> seek::SeekResult<()> {
    let args :Vec<String> = env::args().collect();
    let query :seek::Query = seek::parse_config(&args);
    seek::run(&query)?;
    Ok(())
}

