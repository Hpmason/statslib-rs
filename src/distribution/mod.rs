pub mod normal;

trait Distribution {
    fn cdf(&self, x: f64) -> f64;
    fn pdf(&self, x: f64) -> f64;
    fn mean(&self) -> f64;
    fn median(&self) -> f64;
    fn stdev(&self) -> f64;
}