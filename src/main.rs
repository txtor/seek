use std::env;

fn main() -> seek::SeekResult<()> {
    let args :Vec<String> = env::args().collect();
    match seek::Query::from_args(&args) {
        Ok(query) => seek::run(&query)?,
        Err(e) => eprintln!("seek: {e}")
    };
    Ok(())
}

