pub trait ArithmeticExtensions {
    fn gcd(&self, other: &Self) -> Self;
    fn lcm(&self, other: &Self) -> Self;
}

impl ArithmeticExtensions for u64 {
    fn gcd(&self, other: &Self) -> Self {
        let mut x = *self;
        let mut y = *other;
        while y != 0 {
            let temp = y;
            y = x % y;
            x = temp;
        }
        x
    }

    fn lcm(&self, other: &Self) -> Self {
        let a = *self;
        let b = *other;
        if a == 0 || b == 0 {
            return 0;
        }
        (a / a.gcd(&b)) * b
    }
}
