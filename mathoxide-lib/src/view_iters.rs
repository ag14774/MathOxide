pub struct ContiguousViewIterator {
    curr: usize,
    end: usize,
}

impl ContiguousViewIterator {
    pub fn new(curr: usize, end: usize) -> Self {
        Self { curr, end }
    }
}

impl Iterator for ContiguousViewIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.curr;
        self.curr += 1;

        if curr < self.end {
            Some(curr)
        } else {
            None
        }
    }
}
