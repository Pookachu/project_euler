pub mod primes;

pub use self::primes::sieve;

pub trait NumberTheory {
    fn factors(&self) -> Vec<u64>;
    fn prime_factors(&self) -> Vec<u64>;
    fn is_prime(&self) -> bool;
    fn is_palindrome(&self) -> bool;
    fn gcd(&self, other: &Self) -> Self;
    fn lcm(&self, other: &Self) -> Self;
}

impl NumberTheory for u64 {
    fn factors(&self) -> Vec<u64> {
        let n =*self;
        if n == 0 { return vec![]; }
        if n == 1 { return vec![1]; }

        let mut factors = Vec::new();
        let limit = (n as f64).sqrt() as u64;

        for i in 1..=limit {
            if n % i == 0 {
                factors.push(i);
                if i * i != n {
                    factors.push(n / i);
                }
            }
        }

        factors.sort_unstable();
        factors
    }

    fn is_prime(&self) -> bool {
        let n = *self;
        if n <= 1 { return false; }
        if n <= 3 { return true; }
        if n % 2 == 0 || n % 3 == 0 { return false; } // get divisibility by 2 or 3 out of the way quickly

        let mut i = 5;
        while i * i <= n { // wheel trial division for 6k +/- 1
            if n % i == 0 || n % (i + 2) == 0 { return false; }
            i += 6;
        }
        true
    }

    fn prime_factors(&self) -> Vec<u64> {
        let mut n = *self;
        let mut factors = Vec::new();

        // Handle 2 separately since it's super fast
        while n % 2 == 0 {
            factors.push(2);
            n /= 2;
        }

        // Now look for odd factors
        let mut i = 3;
        // We calculate i*i every iteration because n is SHRINKING
        while i * i <= n {
            while n % i == 0 {
                factors.push(i);
                n /= i;
            }
            i += 2; // Skip even numbers
        }

        // If n > 1, the remaining bit left over is a prime number
        if n > 1 {
            factors.push(n);
        }

        factors

    }

    fn is_palindrome(&self) -> bool {
        let n = *self;
        let mut original = n;
        let mut reversed = 0;

        while original > 0 {
            let digit = original % 10;     // get last digit
            reversed = reversed * 10 + digit;   // append digit to the reversed number
            original /= 10;                     // remove last digit
        }

        n == reversed
    }

    // Euclid's Algorithm for GCD
    fn gcd(&self, other: &u64) -> u64 {
        let mut a = *self;
        let mut b = *other;
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    fn lcm(&self, other: &u64) -> u64 {
        if *self == 0 || *other == 0 {
            return 0;
        }
        // Calculate (a * b) / gcd(a, b)
        (self / self.gcd(other)) * other
    }
    
}