pub trait DigitalExtensions {
    fn is_palindrome(&self) -> bool;
    //fn sum_digits(&self) -> u64;
    //fn is_pandigital(&self) -> bool;
}

impl DigitalExtensions for u64 {
    fn is_palindrome(&self) -> bool {
        let n = *self;
        let mut original = n;
        let mut reversed = 0;

        while original > 0 {
            let digit = original % 10; // Get last digit
            reversed = reversed * 10 + digit; // Append digit to the reversed number
            original /= 10; // Remove last digit
        }

        n == reversed
    }
}
