fn f(x: f64) -> f64 {
    (0.5 * x).exp() + x * x - 3.0
}

fn split(v: Vec<f64>, value: f64) -> (Vec<f64>, Vec<f64>) {
    let (left, right) = v.into_iter().partition(|&x| x < value);
    (left, right.split_at(1).1.to_vec())
}

fn quicksort(v: Vec<f64>) -> Vec<f64> {
    if v.len() <= 1 {
        return v;
    }
    let value = v[0];
    let (left, right) = split(v, value);
    let left_sorted = quicksort(left);
    let right_sorted = quicksort(right);
    [left_sorted, vec![value], right_sorted].concat()
}

fn bisect_f<T: Fn(f64) -> f64>(a: f64, b: f64, f: T) -> (f64, f64) {
    let m = (a + b) * 0.5;
    let (left, mid) = (f(a), f(m));
    if left * mid < 0.0 {
        return (a, m);
    }
    return (m, b);
}

fn main() {
    let value = f(3.0);
    let solution = bisect_f(0., 2., f);
    println!("{solution:?}");
    println!("{value}");
    println!("Hello, world!");

    let v = vec![45., 0., 45., 1., 2., -1., 0., -23., 10.];
    let n = v.len();
    let result = quicksort(v);
    println!("{result:?}");
    assert_eq!(result.len(), n);
}
