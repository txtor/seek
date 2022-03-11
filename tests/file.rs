fn check(filename :&str, query :&seek::Query, expected :&[(u32,&str)]) {
    let searcher = seek::filesearcher::FileSearcher::new(filename, &query).unwrap();
    let got :Vec<seek::filesearcher::FileMatch> = searcher
        .map(|f| f
            .unwrap())
        .collect();
    if got.len() != expected.len() {
        panic!("expected {} files, got {} files",expected.len(),got.len());
    }
    let want :Vec<seek::filesearcher::FileMatch> = expected
        .into_iter()
        .map(|f| seek::filesearcher::FileMatch {
            filename: filename,
            line_number: f.0,
            line: String::from(f.1)
        })
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
    check(&"tests/inputs/dir1/a", &query, &[(1,"alpha beta")]);
}

#[test] 
fn match_line_with_ln() {
    let query = seek::Query::from_strs(&["alpha"]);
    check(&"tests/inputs/dir1/b", &query, &[(1,"alpha beta")]);
}

#[test] 
fn match_once_in_word() {
    let query = seek::Query::from_strs(&["Wald"]);
    check(&"tests/inputs/dir1/c", &query, &[
        (6,"Die Vögelein schweigen im Walde.")
        ]);
}

#[test] 
fn match_twice() {
    let query = seek::Query::from_strs(&["du"]);
    check(&"tests/inputs/dir1/c", &query, &[
        (4,"Spürest du"),
        (8,"Ruhest du auch.")
        ]);
}

#[test] 
fn match_all_content() {
    let query = seek::Query::from_strs(&[]);
    let content = &[
        (1,"Über allen Gipfeln"),
        (2,"Ist Ruh',"),
        (3,"In allen Wipfeln"),
        (4,"Spürest du"),
        (5,"Kaum einen Hauch;"),
        (6,"Die Vögelein schweigen im Walde."),
        (7,"Warte nur, balde"),
        (8,"Ruhest du auch.")
    ];
    check(&"tests/inputs/dir1/c", &query, content);
}
