pub struct FibonacciNumber {
    curr: u64,
    next: u64,
}

impl FibonacciNumber {
    pub fn new() -> Self {
        FibonacciNumber { curr: 0, next: 1 }
    }
}

impl Iterator for FibonacciNumber {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = current + self.next;

        Some(current)
    }
}