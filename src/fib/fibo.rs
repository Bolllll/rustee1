pub struct Fibo {
    pub curr: u32,
    next: u32,
}

impl Fibo {
    pub fn new() -> Self {
        Self {
            curr: 1,
            next: 1,
        }
    }
}
impl Iterator for Fibo {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.curr;
        self.curr = self.next;
        self.next = tmp + self.next;
        Some(tmp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibo() {
        let mut fibo = Fibo::new();
        assert_eq!(fibo.next(), Some(1));
        assert_eq!(fibo.next(), Some(1));
        assert_eq!(fibo.next(), Some(2));
        assert_eq!(fibo.next(), Some(3));
        assert_eq!(fibo.next(), Some(5));
        assert_eq!(fibo.next(), Some(8));
    }
}