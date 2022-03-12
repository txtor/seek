#[derive(Debug)]
pub struct LineChecker {}

impl crate::Checker for LineChecker {
    #[allow(unused_variables)]
    fn check(&self, query :&crate::Query, num :u32, lin :&str) -> bool {
        let mut found :bool = true;
        for target in query.get_targets() {
            if !lin.contains(target) {
                found = false;
                break;
            }
        }
        found   
    }
    fn clear(&self) {}
}
