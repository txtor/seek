use std::env;

fn main() -> seek::Result<()> {
    let args: Vec<String> = env::args().collect();
    let query = if args.len() > 1 {&args[1]} else {""};
    seek::search_dir(".", query)?;
    Ok(())
}
