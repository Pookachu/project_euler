fn main() {
    let raw_data = include_str!("../data/problem_013.txt");
    let mut sum: u64 = 0;

    for line in raw_data.lines() {
        let slice = &line[0..15];

        let num: u64 = slice.parse().unwrap();

        sum += num;
    }

    println!("Total Sum: {}", sum);
    println!("First 10 digits: {}", &sum.to_string()[0..10]);
}
