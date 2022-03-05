use std::env;

fn main() -> seek::Result<()> {
    let args :Vec<String> = env::args().collect();
    let query :&str = seek::parse_config(&args);
    seek::run(query)?;
    Ok(())
}

