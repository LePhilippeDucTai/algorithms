use std::cell::RefCell;

use rand::Rng;
use rand_distr::{Distribution, Uniform};

// Marsaglia Normal simulation

#[derive(Clone, Copy, Debug)]
pub struct UniformUnitDisk;

impl Distribution<(f64, f64)> for UniformUnitDisk {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> (f64, f64) {
        let unif = Uniform::new(-1., 1.).unwrap();
        let a: f64 = rng.sample(unif);
        let y = (1. - a * a).sqrt();
        let cond_unif = Uniform::new(-y, y).unwrap();
        let b = rng.sample(cond_unif);
        (a, b)
    }
}

pub struct BivariateStandardNormal;
pub struct StandardNormal {
    cached_value: RefCell<Option<f64>>,
}

impl Distribution<(f64, f64)> for BivariateStandardNormal {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> (f64, f64) {
        let UniformUnitDisk = UniformUnitDisk;
        let (x, y) = UniformUnitDisk.sample(rng);
        let s = x * x + y * y;
        let r = -2. * s.ln();
        let c = (r / s).sqrt();
        (x * c, y * c)
    }
}

impl Default for StandardNormal {
    fn default() -> Self {
        Self::new()
    }
}

impl StandardNormal {
    pub fn new() -> Self {
        StandardNormal {
            cached_value: RefCell::new(None),
        }
    }
}

impl Distribution<f64> for StandardNormal {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let mut cached_value = self.cached_value.borrow_mut();
        if let Some(value) = *cached_value {
            *cached_value = None;
            value
        } else {
            let biv = BivariateStandardNormal;
            let (x, y) = biv.sample(rng);
            *cached_value = Some(y);
            x
        }
    }
}
