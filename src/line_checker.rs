pub struct LineChecker<'a> {
    pub filesearcher: Option<&'a crate::filesearcher::FileSearcher<'a>>
}

impl<'a> LineChecker<'a> {
    pub fn new() -> Self {
        LineChecker { filesearcher: None }
    }
}

impl<'a> crate::filesearcher::Checker<'a> for LineChecker<'a> {
    #[allow(unused_variables)]
    fn open_search(&mut self, 
        filesearcher: &'a crate::filesearcher::FileSearcher<'a>) 
        -> std::io::Result<()> {
        self.filesearcher = Some(filesearcher);
        Ok(())
    }
    fn check(&self, lin :&str) -> bool {
        let mut found :bool = true;
        for target in self.filesearcher.unwrap().query.get_targets() {
            if !lin.contains(target) {
                found = false;
                break;
            }
        }
        found   
    }
}
