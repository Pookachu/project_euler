// A Pythagorean triplet is a set of three natural numbers, a < b < c for which
// a^2 + b^2 = c^2

// For example, 3^2 + 4^2  = 5^2

// There exists exactly one triplet for which a + b + c = 1000
// Find the product abc

use project_euler::prelude::*;

fn main() {
    let target = 500;
    let divisors = target.divisors(); // [1, 2, 4... 500]

    for m in divisors {
        // We know m(m+n) = 500, so (m+n) = 500/m
        let m_plus_n = 500 / m;

        if m_plus_n > m {
            let n = m_plus_n - m;
            // Now check Euclid's rule: m > n
            if m > n {
                // Found it, generate a, b, c
                let a = m * m - n * n;
                let b = 2 * m * n;
                let c = m * m + n * n;
                println!("Found: a={}, b={}, c={}", a, b, c);
                println!("Product: {}", a * b * c);
                return;
            }
        }
    }
}
