use rand::Rng;
use rand_distr::{Distribution, Open01};
// Marsaglia Normal simulation

#[derive(Clone, Copy, Debug)]
pub struct UniformUnitDisk;


impl Distribution<(f64, f64)> for UniformUnitDisk {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> (f64, f64) {
        let res = std::iter::from_fn(|| {
            let a : f64 = rng.sample(Open01);
            let b : f64 = rng.sample(Open01);
            let x : f64= 2.* a -1.;
            let y :f64 = 2.*b - 1.;
            Some((x,y))
        }).find(|(x, y)| x*x + y*y < 1.).unwrap();
        res
    }
}

// #[time_it]
// pub fn uniform_circle() -> (f64, f64) {
//     let mut rng = rand::rng();
//     let res = std::iter::from_fn(|| {
//         let x= 2.* rng.sample(Open01) -1.;
//         let y = 2.* rng.sample(Open01) -1.;
//         Some((x,y))
//     }).find(|(x, y)| x*x + y*y < 1.).unwrap();
//     res
// }

struct Brownian {
    times: Vec<f64>,
    x: Vec<f64>,
}

pub fn f() {}
