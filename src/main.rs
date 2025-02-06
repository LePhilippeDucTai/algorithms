use algorithms::alg::{bisect, leet_code, sort};
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
    let result = sort::quicksort(v);
    println!("{result:?}");

    // For integers
    let v_int = vec![4, 2, 8, 1, 9, 3];
    let result = sort::quicksort(v_int);
    println!("{result:?}");

    let s = leet_code::two_sum(vec![4, 2, 8, 1, 9, 3], 11);
    println!("{s:?}");
}
