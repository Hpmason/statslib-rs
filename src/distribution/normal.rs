use libm::erf;
use std::f64;

use crate::graph::Graphable;

use super::Distribution;

pub struct NormalDistribution {
    mean: f64,
    stdev: f64,
}

impl NormalDistribution {
    pub fn new(mean: f64, stdev: f64) -> Self {
        Self { mean, stdev }
    }
}
// From https://en.wikipedia.org/wiki/Normal_distribution
impl Distribution for NormalDistribution {
    fn cdf(&self, x: f64) -> f64 {
        (1f64 / 2f64) * (1f64 + erf((x - self.mean) / (self.stdev * 2f64.sqrt())))
    }

    fn pdf(&self, x: f64) -> f64 {
        let inter = 1f64 / (self.stdev * (2f64 * f64::consts::PI).sqrt());
        let expo = (-1f64 / 2f64) * ((x - self.mean) / self.stdev).powi(2);
        inter * expo.exp()
    }

    fn mean(&self) -> f64 {
        self.mean
    }

    fn median(&self) -> f64 {
        self.mean
    }

    fn stdev(&self) -> f64 {
        self.stdev
    }
}

impl Graphable for NormalDistribution {
    fn f(&self, x: f64) -> Option<f64> {
        Some(self.pdf(x))
    }
}

#[cfg(test)]
mod tests {
    use super::NormalDistribution;
    use crate::{assert_close, distribution::Distribution};

    #[test]
    fn test_std_normal_pdf() {
        let std_normal = NormalDistribution::new(0f64, 1f64);
        assert_close!(std_normal.pdf(-2f64), 0.05399097f64, 0.00000001);
        assert_close!(std_normal.pdf(-1f64), 0.24197072f64, 0.00000001);
        assert_close!(std_normal.pdf(0f64), 0.39894228f64, 0.00000001);
        assert_close!(std_normal.pdf(1f64), 0.24197072f64, 0.00000001);
        assert_close!(std_normal.pdf(2f64), 0.05399097f64, 0.00000001);
    }
}
