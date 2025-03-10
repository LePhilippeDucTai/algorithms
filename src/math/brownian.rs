use itertools::Itertools;
use rand::Rng;
use rand_distr::{Normal, StandardNormal};

pub struct IterProcess<'a, P: RandomProcess, R: Rng + ?Sized> {
    process: &'a P,
    rng: &'a mut R,
    log2n: u8,
    terminal_date: f64,
}

impl<'a, P: RandomProcess, R: Rng + ?Sized> Iterator for IterProcess<'a, P, R> {
    type Item = Vec<(f64, f64)>;
    fn next(&mut self) -> Option<Self::Item> {
        let trajectory = self
            .process
            .sample(&mut self.rng, self.log2n, self.terminal_date);
        Some(trajectory)
    }
}

pub trait RandomProcess {
    fn sample<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        log2n: u8,
        terminal_date: f64,
    ) -> Vec<(f64, f64)>;

    fn sample_iter<'a, R: Rng + ?Sized>(
        &'a self,
        rng: &'a mut R,
        log2n: u8,
        terminal_date: f64,
    ) -> IterProcess<'a, Self, R>
    where
        Self: Sized,
    {
        IterProcess {
            process: self,
            rng,
            log2n,
            terminal_date,
        }
    }
}

pub struct BrownianProcess;

fn sample_midpoint<R: Rng + ?Sized>(rng: &mut R, v: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let w = v
        .iter()
        .tuple_windows()
        .map(|((t1, x1), (t2, x2))| {
            let t_mid = (t1 + t2) * 0.5;
            let mid = (x1 + x2) * 0.5;
            let std = ((t2 - t1) * 0.25).sqrt();
            let distr: Normal<f64> = Normal::new(mid, std).unwrap();
            (t_mid, rng.sample(distr))
        })
        .collect_vec();
    w
}

fn brownian_bridge_iter<R: Rng + ?Sized>(v: Vec<(f64, f64)>, rng: &mut R) -> Vec<(f64, f64)> {
    let s: Vec<(f64, f64)> = sample_midpoint(rng, &v);
    let it: Vec<(f64, f64)> = v.iter().interleave(s.iter()).map(|a| *a).collect();
    it
}

impl RandomProcess for BrownianProcess {
    fn sample<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        log2n: u8,
        terminal_date: f64,
    ) -> Vec<(f64, f64)> {
        let terminal_value: f64 = rng.sample::<f64, _>(StandardNormal) * terminal_date.sqrt();
        let v: Vec<(f64, f64)> = vec![(0., 0.), (terminal_date, terminal_value)];
        let brownian: Vec<(f64, f64)> =
            (0..log2n).fold(v, |acc: Vec<(f64, f64)>, _| brownian_bridge_iter(acc, rng));
        brownian
    }
}
