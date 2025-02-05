pub fn bisect_f<T: Fn(f64) -> f64>(a: f64, b: f64, f: T) -> (f64, f64) {
    let m = (a + b) * 0.5;
    let (left, mid) = (f(a), f(m));
    if left * mid < 0.0 {
        return (a, m);
    }
    return (m, b);
}
