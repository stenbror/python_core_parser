use std::fmt;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location {
    pub(super) start_pos: u32,
    pub(super) end_pos: u32
}

impl Location {
    fn new(s: usize, e: usize) -> Self {
        Location { 
            start_pos: s.try_into().expect("Start position for current element!"), 
            end_pos: e.try_into().expect("SEnd position for current element!")
        }
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( start: {} end: {} )", self.start_pos, self.end_pos)
    }

    fn start(&self) -> usize {
        self.start_pos as usize
    }

    fn end(&self) -> usize {
        self.end_pos as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::location::Location;

    #[test]
    fn test_gt() {
        assert!(Location::new(1, 2) > Location::new(1, 1));
        assert!(Location::new(2, 1) > Location::new(1, 1));
        assert!(Location::new(2, 1) > Location::new(1, 2));
    }

    #[test]
    fn test_lt() {
        assert!(Location::new(1, 1) < Location::new(1, 2));
        assert!(Location::new(1, 1) < Location::new(2, 1));
        assert!(Location::new(1, 2) < Location::new(2, 1));
    }
}