pub mod arithmetic;
pub mod combinatorics;
pub mod digits;
pub mod primes;

pub use arithmetic::ArithmeticExtensions;
pub use combinatorics::CombinatoricsExtensions;
pub use digits::DigitalExtensions;
pub use primes::PrimeExtensions;
pub trait NumberTheory:
    PrimeExtensions + CombinatoricsExtensions + DigitalExtensions + ArithmeticExtensions
{
}

impl<T> NumberTheory for T where
    T: PrimeExtensions + ArithmeticExtensions + DigitalExtensions + CombinatoricsExtensions
{
}
