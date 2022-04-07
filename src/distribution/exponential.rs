use crate::graph::Graphable;

use super::Distribution;

pub struct ExponentialDistribution {
    lambda: f64,
}

impl ExponentialDistribution {
    pub fn new(lambda: f64) -> Self {
        Self { lambda }
    }
    pub fn from_mean(mean: f64) -> Self {
        Self {
            lambda: 1f64 / mean,
        }
    }
    pub fn from_stdev(stdev: f64) -> Self {
        Self {
            lambda: 1f64 / stdev,
        }
    }
}

impl Distribution for ExponentialDistribution {
    fn cdf(&self, x: f64) -> f64 {
        if x >= 0f64 {
            1f64 - (-self.lambda * x).exp()
        } else {
            0f64
        }
    }

    fn pdf(&self, x: f64) -> f64 {
        if x >= 0f64 {
            self.lambda * (-self.lambda * x).exp()
        } else {
            0f64
        }
    }

    fn mean(&self) -> f64 {
        1f64 / self.lambda
    }

    fn median(&self) -> f64 {
        2f64.ln() / self.lambda
    }

    fn stdev(&self) -> f64 {
        self.mean()
    }
}
impl Graphable for ExponentialDistribution {
    fn f(&self, x: f64) -> Option<f64> {
        Some(self.pdf(x))
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_close, distribution::Distribution};

    use super::ExponentialDistribution;

    #[test]
    fn test_expo() {
        let expo_dist = ExponentialDistribution::from_mean(10f64);
        assert_close!(expo_dist.mean(), 10f64);
        assert_close!(expo_dist.median(), 2f64.ln() * 10f64);
        assert_close!(expo_dist.stdev(), 10f64);
        assert_close!(expo_dist.pdf(0f64), 0.1f64);
        assert_close!(expo_dist.pdf(1f64), 0.090_483_742f64);
        assert_close!(expo_dist.pdf(-1f64), 0f64);
    }
}
