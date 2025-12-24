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
    is_prime.iter()
        .enumerate()
        .filter_map(|(num, &is_p)| if is_p { Some(num as u64) } else { None })
        .collect()
}