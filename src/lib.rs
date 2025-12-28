pub mod number_theory;
pub mod sequences;

pub use number_theory::arithmetic;
pub use number_theory::combinatorics;
pub use number_theory::digits;
pub use number_theory::primes;

pub mod prelude {
    pub use crate::number_theory::ArithmeticExtensions;
    pub use crate::number_theory::CombinatoricsExtensions;
    pub use crate::number_theory::DigitalExtensions;
    pub use crate::number_theory::NumberTheory;
    pub use crate::number_theory::PrimeExtensions;
}

pub use number_theory::NumberTheory;
pub use number_theory::primes::sieve;
