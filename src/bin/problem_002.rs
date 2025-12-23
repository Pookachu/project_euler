use project_euler::sequences::FibonacciNumber;

fn main() {

    let sum: u64 = FibonacciNumber::new()
        .take_while(|&x| x < 4_000_000) // Stop at 4 million
        .filter(|&x| x % 2 == 0)        // Keep only even numbers
        .sum();                                                        // Add them up

    println!("Found: {}", sum);
}