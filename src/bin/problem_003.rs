use project_euler::NumberTheory;

fn main() {
    let number: u64 = 600851475143;

    let largest = *number.prime_factors().last().unwrap(); 

    println!("Found: {}", largest);

}