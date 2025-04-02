pub(crate) trait LogWalker<Id> {
    fn read(&mut self, out: &mut Vec<Id>) -> usize;
}
