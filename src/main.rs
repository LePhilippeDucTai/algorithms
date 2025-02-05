// use algorithms::algorithms::{bisect, sort};

use algorithms::alg::{bisect, sort};

fn f(x: f64) -> f64 {
    (0.5 * x).exp() + x * x - 3.0
}

fn main() {
    let value = f(3.0);
    let solution = bisect::bisect_f(0., 2., f);
    println!("{solution:?}");
    println!("{value}");
    println!("Hello, world!");

    let v = vec![45., 0., 45., 1., 2., -1., 0., -23., 10.];
    let n = v.len();
    let result = sort::quicksort(v);
    println!("{result:?}");
    assert_eq!(result.len(), n);
}
