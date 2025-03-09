use itertools::Itertools;
use rand::Rng;
use rand_distr::{Normal, StandardNormal};

struct Brownian {
    x0: f64,
}

impl Brownian {
    fn default(&self) -> Brownian {
        Brownian { x0: 0.0 }
    }
}

struct BrownianProcess {
    t: Vec<f64>,
    x: Vec<f64>,
}

// impl<'a> IterableProcess for BrownianProcess<'a> {
//     #[inline(always)]
//     fn t(&self) -> &Vec<f64> {
//         &self.t
//     }
//     #[inline(always)]
//     fn x(&self) -> &Vec<f64> {
//         &self.x
//     }
// }

impl Brownian {
    fn sample<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        log2n: u8,
        terminal_date: f64,
    ) -> BrownianProcess {
        let terminal_value: f64 = rng.sample::<f64, _>(StandardNormal) * terminal_date;
        let dates = vec![0.0, terminal_date];
        let values = vec![self.x0, terminal_value];
        let points_v = vec![(0.0, self.x0), (terminal_date, terminal_value)];
        let points_w = points_v.clone();
        let inter_points = points_v
            .into_iter()
            .tuple_windows()
            .map(|((t1, x1), (t2, x2))| {
                let t_mid = (t1 + t2) * 0.5;
                let mid = (x1 + x2) * 0.5;
                let std = ((t2 - t1) * 0.25).sqrt();
                let distr = Normal::new(mid, std).unwrap();
                (t_mid, rng.sample(distr))
            });
        let result_points = points_w.into_iter().interleave(inter_points).collect_vec();
        BrownianProcess {
            t: dates,
            x: values,
        }
    }
}
