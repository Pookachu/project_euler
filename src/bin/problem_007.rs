use project_euler::sieve;

fn main() {
    let target_index: usize = 10_001;
    
    // Find upper bound for the nth prime 
    // A reliable upper bound for p_n when n \geq 6 is n \times (\ln n) + \ln (\ln (n) ))
    let n = target_index as f64;
    let limit = (n * (n.ln() + n.ln().ln())) as u64;

    // Apply Sieve of Eratosthenes to find all primes to the upper bound
    let primes = sieve(limit);

    // -1 to account for 0 index
    let answer: u64 = primes[target_index - 1]; 

    println!("Found the {}th prime: {}",target_index, answer);
}