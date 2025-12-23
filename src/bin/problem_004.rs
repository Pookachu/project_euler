use project_euler::NumberTheory;

fn main() {
    let mut largest_palindrome = 0;

    for i in (1..1000).rev() {
        for j in (1..1000).rev() {
            let product = i*j;

            if product < largest_palindrome {
                break;
            }

            if product.is_palindrome() {
                largest_palindrome = product;
            }
        }
    }

    println!("Found: {}", largest_palindrome);
}