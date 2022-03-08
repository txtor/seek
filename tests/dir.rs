
fn check(dir :&str, recur :bool, expected :&[&str]) -> seek::SeekResult<()> {
    let searcher = seek::dirsearcher::DirSearcher::new(dir, recur)?;
    let mut got :Vec<String> = searcher
        .map(|f| f
            .path()
            .to_string_lossy()
            .into_owned())
        .collect();
    if got.len() != expected.len() {
        panic!("expected {} files, got {} files",expected.len(),got.len());
    }
    let mut want :Vec<String> = expected
        .into_iter()
        .map(|f| format!("{}/{}",dir,*f))
        .collect();
    got.sort();
    want.sort();
    for i in 0..got.len() {
        if got[i] != want[i] {
            panic!("expected {}, got {}",want[i],got[i]);
        }
    }
    Ok(())
}

#[test] 
fn empty_dir() -> seek::SeekResult<()> {
    check("tests/inputs/dir2", true, &[])
}

#[test] 
#[should_panic]
fn unexisting_dir() {
    let searcher = seek::dirsearcher::DirSearcher::new("tests/inputs/dir0", true).unwrap();
}

#[test]
fn dirs_dir1() -> seek::SeekResult<()> {
    check("tests/inputs/dir1", false, &["a","b","c","d","subdir1","subdir2"])
}

#[test]
#[should_panic]
fn check_fail() {
    check("tests/inputs/dir1", false, &["a","b","C","d","subdir1","subdir2"]).unwrap();
}

#[test] 
fn dirs_dir1_recur() -> seek::SeekResult<()> {
    check("tests/inputs/dir1", true, &["a","b","c","d",
        "subdir1",
        "subdir1/e",
        "subdir1/f",
        "subdir2",
        "subdir2/g",
        "subdir2/h",
        "subdir2/i",
        "subdir2/subdir3",
        "subdir2/subdir3/l",
        "subdir2/subdir3/k",
        "subdir2/subdir3/j",
        "subdir2/subdir3/m",
        ])
}
