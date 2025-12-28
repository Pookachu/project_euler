use project_euler::number_theory::primes::sieve;

fn main() {
    let limit: u64 = 2_000_000;

    let primes = sieve(limit);

    let answer: u64 = primes.iter().sum();

    println!("Found: {}", answer);
}
