use crate::graph::Graphable;

use super::Distribution;

pub struct UniformDistribution {
    a: f64,
    b: f64,
}

impl UniformDistribution {
    pub fn new(a: f64, b: f64) -> Self {
        assert!(a <= b);
        Self { a, b }
    }
}

impl Distribution for UniformDistribution {
    fn cdf(&self, x: f64) -> f64 {
        if x < self.a {
            0f64
        } else if self.a <= x && x <= self.b {
            (x - self.a) / (self.b - self.a)
        } else {
            0f64
        }
    }

    fn pdf(&self, x: f64) -> f64 {
        if self.a <= x && x <= self.b {
            1f64 / (self.b - self.a)
        } else {
            0f64
        }
    }

    fn mean(&self) -> f64 {
        (self.b + self.a) / 2f64
    }

    fn median(&self) -> f64 {
        self.mean()
    }

    fn stdev(&self) -> f64 {
        (self.b - self.a) / 12f64.sqrt()
    }
}

impl Graphable for UniformDistribution {
    fn f(&self, x: f64) -> Option<f64> {
        if self.a <= x && x <= self.b {
            Some(1f64 / (self.b - self.a))
        } else {
            Some(0f64)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_close, distribution::Distribution};

    use super::UniformDistribution;

    #[test]
    fn test_uniform_dist() {
        let uni_dist = UniformDistribution::new(0f64, 10f64);
        assert_close!(uni_dist.mean(), 5f64);
        assert_close!(uni_dist.median(), 5f64);
        assert_close!(uni_dist.stdev(), 2.886_751_346f64);
        assert_close!(uni_dist.pdf(5f64), 1f64 / 10f64);
    }
    #[test]
    fn test_uniform_dist_with_negative() {
        let uni_dist = UniformDistribution::new(-5f64, -1f64);
        assert_close!(uni_dist.mean(), -3f64);
        assert_close!(uni_dist.median(), -3f64);
        assert_close!(uni_dist.stdev(), 1.154_700_538f64);
        assert_close!(uni_dist.pdf(-3f64), 1f64 / 4f64);
    }
    #[test]
    fn test_uniform_dist_with_neg_and_pos() {
        let uni_dist = UniformDistribution::new(-5f64, 5f64);
        assert_close!(uni_dist.mean(), 0f64);
        assert_close!(uni_dist.median(), 0f64);
        assert_close!(uni_dist.stdev(), 2.886_751_346f64);
        assert_close!(uni_dist.pdf(0f64), 1f64 / 10f64);
    }
}
