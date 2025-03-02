use rand::{distr::Uniform, Rng};
// Marsaglia Normal simulation
pub fn uniform_circle() -> (f64, f64) {
    let unif = Uniform::new(-1., 1.);
    let mut rng = rand::rng();
    let x = rng.sample(unif.unwrap());
    let xsq = x * x;
    let y = rng
        .sample_iter(unif.unwrap())
        .find(|z| z * z + xsq < 1.)
        .unwrap();
    return (x, y);
}

struct Brownian {
    times: Vec<f64>,
    x: Vec<f64>,
}

pub fn f() {}
