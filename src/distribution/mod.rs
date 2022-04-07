pub mod exponential;
pub mod normal;
pub mod uniform;

pub trait Distribution {
    fn cdf(&self, x: f64) -> f64;
    fn pdf(&self, x: f64) -> f64;
    fn mean(&self) -> f64;
    fn median(&self) -> f64;
    fn stdev(&self) -> f64;
}

#[cfg(test)]
#[macro_export]
/// Assert numbers are close together.
/// If third parameter not provided, it will check for 9 decimal points of precision
/// ```rust
/// let x = 0.123_456_789;
/// let y = 0.123_456_789_10;
/// assert_close!(x, y) // True
/// ```
///
/// ```rust
///
/// let a = 1f64 / 16f64; // 0.0625
/// let b = 0.0625f64;
/// // Only check precision up to the 4th decimal
/// assert_close!(a, b, 0.0001); // True
/// ```
macro_rules! assert_close {
    ($a:expr, $b:expr, $prox:expr) => {
        if !(($b - $a).abs() < $prox) {
            panic!("{} !~ {} with a approximation of {}", $a, $b, $prox)
        }
    };
    ($a:expr, $b:expr) => {
        assert_close!($a, $b, 1e-9)
    };
}
