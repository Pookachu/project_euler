use project_euler::prelude::*;

fn main() {
    // Create the infinite iterator of triangle numbers
    let mut triangle_numbers = (1u64..).scan(0, |state, x| {
        *state += x;
        Some(*state)
    });

    // Use .find() to get the first one with > 500 factors
    let result = triangle_numbers.find(|n| n.divisor_count() > 500);

    match result {
        Some(n) => println!("Found it: {}", n),
        None => println!("Reached integer limit without finding one."),
    }
}
