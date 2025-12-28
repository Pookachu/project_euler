pub trait CombinatoricsExtensions {
    fn factorial(&self) -> u64;
    fn n_cr(&self, r: u64) -> u64;
    fn n_pr(&self, r: u64) -> u64;
}
