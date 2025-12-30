pub trait CombinatoricsExtensions {
    fn factorial(&self) -> u64;
    fn n_cr(&self, k: u64) -> u64;
    fn n_pr(&self, r: u64) -> u64;
}

impl CombinatoricsExtensions for u64 {
    fn factorial(&self) -> u64 {
        (1..=*self).product()
    }

    fn n_cr(&self, k: u64) -> u64 {
        if k > *self {
            return 0;
        }
        let k = if k > *self - k { *self - k } else { k }; // Symmetry optimization

        let mut res: u128 = 1; // Use u128 to prevent intermediate overflows
        for i in 0..k {
            res = res * (*self as u128 - i as u128) / (i as u128 + 1);
        }
        res as u64
    }

    fn n_pr(&self, _r: u64) -> u64 {
        0 // placeholder TODO
    }
}
