use std::collections::BTreeMap;

pub trait PrimeExtensions {
    fn is_prime(&self) -> bool;
    fn prime_factors(&self) -> Vec<u64>;
    fn prime_signature(&self) -> BTreeMap<u64, u32>;
    fn divisor_count(&self) -> u64;
    fn divisors(&self) -> Vec<u64>;
}

impl PrimeExtensions for u64 {
    fn is_prime(&self) -> bool {
        let n = *self;
        if n <= 1 {
            return false;
        }
        if n <= 3 {
            return true;
        }
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        } // get divisibility by 2 or 3 out of the quickly

        let mut i = 5;
        while i * i <= n {
            // wheel trial division for 6k +/- 1
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }

    fn prime_factors(&self) -> Vec<u64> {
        let mut n = *self;
        let mut factors = Vec::new();
        while n % 2 == 0 {
            factors.push(2);
            n /= 2;
        }
        let mut i = 3;
        while i * i <= n {
            while n % i == 0 {
                factors.push(i);
                n /= i;
            }
            i += 2;
        }
        if n > 1 {
            factors.push(n);
        }
        factors
    }

    fn prime_signature(&self) -> BTreeMap<u64, u32> {
        let n = *self;
        let mut signature = BTreeMap::new();
        let factors = n.prime_factors();
        for prime in factors {
            *signature.entry(prime).or_insert(0) += 1;
        }
        signature
    }

    fn divisor_count(&self) -> u64 {
        let factors = self.prime_factors();

        if factors.is_empty() {
            return 1; // Case for number 1
        }

        let mut total_divisors = 1;
        let mut current_exponent = 1;

        // Iterate starting from the second element
        for i in 1..factors.len() {
            if factors[i] == factors[i - 1] {
                // Found a duplicate, increment the exponent for this prime
                current_exponent += 1;
            } else {
                // Found a new prime.
                // 1. Multiply the running total by (exponent + 1)
                total_divisors *= current_exponent + 1;
                // 2. Reset exponent counter for the new prime
                current_exponent = 1;
            }
        }

        // Don't forget to multiply the last group!
        total_divisors *= current_exponent + 1;

        total_divisors
    }

    fn divisors(&self) -> Vec<u64> {
        // 1. Get the building blocks (e.g., {2: 2, 5: 3} for 500)
        let signature = self.prime_signature();

        // 2. Start with just 1
        let mut divisors = vec![1];

        // 3. Iterate over each prime factor
        for (prime, exponent) in signature {
            let mut new_combinations = Vec::new();

            // For every existing divisor, multiply it by p^1, p^2, ... p^k
            for d in &divisors {
                let mut prime_power = 1;
                for _ in 0..exponent {
                    prime_power *= prime;
                    new_combinations.push(d * prime_power);
                }
            }

            // Add the new numbers to our list
            divisors.extend(new_combinations);
        }

        // 4. Sort them for convenience (optional, but usually expected)
        divisors.sort();
        divisors
    }
}

/// Generates a list of all prime numbers up to (and including) the limit.
/// Uses the Sieve of Eratosthenes.
pub fn sieve(limit: u64) -> Vec<u64> {
    if limit < 2 {
        return vec![];
    }

    let limit_usize = limit as usize;

    // 1. Create the boolean array (true = prime, false = composite)
    // We add +1 so index 'n' corresponds to number 'n'
    let mut is_prime = vec![true; limit_usize + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    // 2. The Sieve Logic
    let sieve_limit = (limit as f64).sqrt() as usize;
    for number in 2..=sieve_limit {
        if is_prime[number] {
            // Start at number * number to avoid redundant checks
            // Use step_by for clean iteration
            for multiple in (number * number..=limit_usize).step_by(number) {
                is_prime[multiple] = false;
            }
        }
    }

    // 3. Convert the boolean map into a list of numbers
    is_prime
        .iter()
        .enumerate()
        .filter_map(|(num, &is_p)| if is_p { Some(num as u64) } else { None })
        .collect()
}
