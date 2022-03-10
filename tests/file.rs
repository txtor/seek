fn check(filename :&str, query :&seek::Query, expected :&[&str]) {
    let searcher = seek::filesearcher::FileSearcher::new(filename, &query).unwrap();
    let got :Vec<String> = searcher
        .map(|f| f
            .unwrap()
            .line)
        .collect();
    if got.len() != expected.len() {
        panic!("expected {} files, got {} files",expected.len(),got.len());
    }
    let want :Vec<String> = expected
        .into_iter()
        .map(|f| String::from(*f))
        .collect();
    for i in 0..got.len() {
        if got[i] != want[i] {
            panic!("expected >{}<, got >{}<",want[i],got[i]);
        }
    }
}

#[test] 
fn no_match() {
    let query = seek::Query::from_strs(&["blah"]);
    check(&"tests/inputs/dir1/a", &query, &[]);
}

#[test] 
fn match_line_without_ln() {
    let query = seek::Query::from_strs(&["alpha"]);
    check(&"tests/inputs/dir1/a", &query, &["alpha beta"]);
}

#[test] 
fn match_line_with_ln() {
    let query = seek::Query::from_strs(&["alpha"]);
    check(&"tests/inputs/dir1/b", &query, &["alpha beta"]);
}

#[test] 
fn match_once_in_word() {
    let query = seek::Query::from_strs(&["Wald"]);
    check(&"tests/inputs/dir1/c", &query, &["Die Vögelein schweigen im Walde."]);
}

#[test] 
fn match_twice() {
    let query = seek::Query::from_strs(&["du"]);
    check(&"tests/inputs/dir1/c", &query, &["Spürest du","Ruhest du auch."]);
}

#[test] 
fn match_all_content() {
    let query = seek::Query::from_strs(&[]);
    let content = &[
        "Über allen Gipfeln",
        "Ist Ruh',",
        "In allen Wipfeln",
        "Spürest du",
        "Kaum einen Hauch;",
        "Die Vögelein schweigen im Walde.",
        "Warte nur, balde",
        "Ruhest du auch."
    ];
    check(&"tests/inputs/dir1/c", &query, content);
}
