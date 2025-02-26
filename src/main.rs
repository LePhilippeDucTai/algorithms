use algorithms::alg::{
    bisect, leet_code,
    sort::{self},
};
use std::time::Instant;
fn f(x: f64) -> f64 {
    (0.5 * x).exp() + x * x - 3.0
}

fn main() {
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
    println!("{s:?}");

    let a = vec![1, 2, 3, 5, 6];
    let b = vec![2, 3, 5, 9, 10];
    let m = sort::mergesorted(a, b);
    println!("{m:?}");

    let x = vec![0, 1, 2, 3, 19, 20, 31];
    let sorted = sort::mergesort(x);
    println!("{sorted:?}");

    let s = sort::linearsearch(&sorted, 32);
    println!("{s}");

    let res = leet_code::isqrt(144);
    println!("{}", res);

    let before = Instant::now();
    let pi = leet_code::compute_pi();
    println!("Elapsed time: {:.2?}", before.elapsed());
    println!("{pi}")
}
