use algorithms::{
    alg::{
        bisect, leet_code,
        sort::{self},
    },
    math::{
        self,
        brownian::{BrownianProcess, RandomProcess},
    },
};
use itertools::Itertools;
use plotly::{Plot, Scatter};
use rand::prelude::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::time::Instant;
use time_it_macro::time_it;
use tracing::info;
use tracing_subscriber;

fn f(x: f64) -> f64 {
    (0.5 * x).exp() + x * x - 3.0
}

#[time_it]
fn calculate_pi(n: usize) -> f64 {
    let counter = (0..n)
        .into_par_iter()
        .map_init(
            || rand::rng(),
            |rng, _| {
                let x: f64 = rng.random();
                let y: f64 = rng.random();
                x * x + y * y < 1.0
            },
        )
        .filter(|&x| x)
        .count();
    4.0 * (counter as f64) / (n as f64)
}

fn main() {
    tracing_subscriber::fmt::init();
    let value = f(3.0);
    let solution = bisect::bisect(f, 0., 2., 0.0001);
    println!("{solution:?}");
    println!("{value}");
    println!("Hello, world!");

    let v = vec![45., 0., 45., 1., 2., -1., 0., -23., 10.];
    let result = sort::quicksort(v);
    println!("{result:?}");

    // For integers
    let v_int = vec![4, 2, 8, 1, 9, 3];
    let result = sort::quicksort(v_int);
    println!("{result:?}");

    let s = leet_code::two_sum(vec![4, 2, 8, 1, 9, 3], 11);
    info!("{s:?}");

    let a = vec![1, 2, 3, 5, 6];
    let b = vec![2, 3, 5, 9, 10];
    let m = sort::mergesorted(a, b);
    println!("{m:?}");

    let x = vec![0, 1, 2, 3, 19, 20, 31];
    let sorted = sort::mergesort(x);
    println!("{sorted:?}");

    let s = sort::linearsearch(&sorted, 32);
    info!("{s}");

    let res = leet_code::isqrt(144);
    info!("{}", res);

    let pi = leet_code::compute_pi(1_000_000);
    info!("{pi}");
    let pi = calculate_pi(1_000_000);
    info!("{pi}");

    let unif_disk = math::random::UniformUnitDisk;
    let mut rng = rand::rng();

    let start = Instant::now();
    let n_sim = 1_000_000;
    let _v = unif_disk.sample_iter(&mut rng).take(n_sim).collect_vec();
    let duration = start.elapsed();
    info!("UniformUnitDisk {n_sim} est de : {:?}", duration);

    let normal_biv_std = math::random::BivariateStandardNormal;
    let start = Instant::now();
    let _v = normal_biv_std
        .sample_iter(&mut rng)
        .take(n_sim)
        .collect_vec();
    let duration = start.elapsed();
    info!("Bivariate Normal {n_sim} est de : {:?}", duration);

    let normal_std = math::random::StandardNormal::new();
    let start = Instant::now();
    let _v = normal_std.sample_iter(&mut rng).take(n_sim).collect_vec();
    let duration = start.elapsed();
    info!("Univariate Normal {n_sim} est de : {:?}", duration);

    let brownian = BrownianProcess;
    let mut srng = StdRng::from_seed([5; 32]);
    let trajectory = brownian.sample(&mut srng, 8, 100.);
    let (t, x) = trajectory.iter().map(|&(t, x)| (t, x)).unzip();
    println!("{trajectory:?}");
    let mut plot = Plot::new();
    let trace = Scatter::new(t, x);
    plot.add_trace(trace);
    plot.write_html("out.html");

    let mut plot = Plot::new();
    let _: () = brownian
        .sample_iter(&mut srng, 10, 1.)
        .take(1_000)
        .for_each(|trajectory| {
            let (t, x) = trajectory.iter().map(|&(t, x)| (t, x)).unzip();
            let trace = Scatter::new(t, x);
            plot.add_trace(trace);
        });
    // plot.write_html("out.html");
}
