use std::cell::RefCell;

use rand::Rng;
use rand_distr::{Distribution, Open01};
// Marsaglia Normal simulation

#[derive(Clone, Copy, Debug)]
pub struct UniformUnitDisk;

impl Distribution<(f64, f64)> for UniformUnitDisk {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> (f64, f64) {
        let res = std::iter::from_fn(|| {
            let a: f64 = rng.sample(Open01);
            let b: f64 = rng.sample(Open01);
            let x: f64 = 2. * a - 1.;
            let y: f64 = 2. * b - 1.;
            Some((x, y))
        })
        .find(|(x, y)| x * x + y * y < 1.)
        .unwrap();
        res
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

pub struct Brownian {
    times: Vec<f64>,
    x: Vec<f64>,
}

impl Brownian {
    fn new() -> Self {
        Brownian {
            times: vec![0.],
            x: vec![0.],
        }
    }
}

pub fn brownian_build() {
    let b = Brownian::new();
    println!("{:?}", b.times);
    println!("{:?}", b.x);
}
