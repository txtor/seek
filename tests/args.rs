
fn get_args(args :&[&str]) -> Vec<String> {
    (*args).into_iter().map(|a| String::from(*a)).collect()
}

#[test] 
fn args_none() -> seek::SeekResult<()> {
    let args :Vec<String> = get_args(&[""]);
    let query = seek::Query::from_args(&args);
    assert_eq!(query.get_targets().len(),0);
    Ok(())
}

#[test] 
fn args_one() -> seek::SeekResult<()> {
    let args :Vec<String> = get_args(&["","main"]);
    let query = seek::Query::from_args(&args);
    assert_eq!(query.get_targets().len(), 1);
    Ok(())
}

#[test] 
fn args_two() -> seek::SeekResult<()> {
    let args :Vec<String> = get_args(&["","blah","main"]);
    let query = seek::Query::from_args(&args);
    assert_eq!(query.get_targets().len(), 2);
    Ok(())
}

#[test] 
fn strs_none() -> seek::SeekResult<()> {
    let query = seek::Query::from_strs(&[]);
    assert_eq!(query.get_targets().len(),0);
    Ok(())
}

#[test] 
fn strs_one() -> seek::SeekResult<()> {
    let query = seek::Query::from_strs(&["main"]);
    assert_eq!(query.get_targets().len(), 1);
    Ok(())
}


#[test] 
fn strs_two() -> seek::SeekResult<()> {
    let query = seek::Query::from_strs(&["blah","void"]);
    assert_eq!(query.get_targets().len(), 2);
    Ok(())
}

