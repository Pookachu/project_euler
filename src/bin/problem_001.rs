fn main() {
    let mut sum = 0;

    for i in 1..1000 {             // Go over every number from 1 to 1000
        if i % 5 == 0 || i % 3 == 0 {   // Check if they're divisible by 3 or 5
            sum += i;                   // Add them to running total
        }
    }

    println!("Found: {}", sum);
}