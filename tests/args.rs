
fn get_args(args :&[&str]) -> Vec<String> {
    (*args).into_iter().map(|a| String::from(*a)).collect()
}

#[test] 
fn args_none() -> seek::SeekResult<()> {
    let args :Vec<String> = get_args(&[""]);
    let query :seek::Query = seek::parse_config(&args);
    assert_eq!(query.get_targets().len(),0);
    Ok(())
}

#[test] 
fn args_one() -> seek::SeekResult<()> {
    let args :Vec<String> = get_args(&["","main"]);
    let query :seek::Query = seek::parse_config(&args);
    assert_eq!(query.get_targets().len(), 1);
    Ok(())
}

