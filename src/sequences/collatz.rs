pub struct CollatzSequence {
    current: u64,
}

impl CollatzSequence {
    pub fn new(start: u64) -> Self {
        CollatzSequence { current: start }
    }

    // Helper logic (no longer needs to be public or used in new)
    fn next_collatz(n: u64) -> u64 {
        if n % 2 == 0 { n / 2 } else { 3 * n + 1 }
    }
}

impl Iterator for CollatzSequence {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // 1. Check if we are done
        // If current is 0, we have already finished the sequence
        if self.current == 0 {
            return None;
        }

        let ret = self.current;

        // 2. Determine the next state
        if ret == 1 {
            // We just yielded 1, so the sequence is over.
            // Set to 0 to signal "stop" for the NEXT call.
            self.current = 0;
        } else {
            // Calculate the next number in the sequence
            self.current = Self::next_collatz(ret);
        }

        Some(ret)
    }
}
