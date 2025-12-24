fn main() {
    let n: u64 = 100;

    // Formula: n(n+1)(2n+1) / 6
    let sum_of_squares = (n * (n + 1) * (2 * n + 1)) / 6;

    // (Sum of 1..100)^2
    let sum = (n * (n + 1)) / 2;
    let square_of_sums = sum * sum;

    let answer = square_of_sums - sum_of_squares;

    println!("Found: {}", answer);


}