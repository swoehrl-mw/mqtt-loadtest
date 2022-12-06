pub struct CyclicRangeIterator {
    start: u64,
    end: u64,
    next: u64,
}

impl CyclicRangeIterator {
    pub fn new(start: u64, end: u64) -> Self {
        CyclicRangeIterator {
            start,
            end,
            next: start,
        }
    }
}

impl Iterator for CyclicRangeIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next;
        if self.next + 1 > self.end {
            self.next = self.start;
        } else {
            self.next += 1;
        }
        Some(current)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cyclicrange() {
        let mut iterator = CyclicRangeIterator::new(1, 3);
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), Some(2));
    }
}
