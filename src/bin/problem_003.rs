use project_euler::prelude::*;

fn main() {
    let number: u64 = 600851475143;

    let largest = *number.prime_factors().last().unwrap();

    println!("Found: {}", largest);
}
