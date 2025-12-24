use project_euler::NumberTheory;
fn main() {

    // We want the LCM of numbers 1 to 20.
    // .fold(1, ...) starts with '1' as the accumulator.
    // It takes the accumulator, calculates LCM with the next number, 
    // and updates the accumulator.

    let answer: u64 = (1..=20).fold(1, |acc, x| acc.lcm(&x));

    println!("Found: {}", answer)
}