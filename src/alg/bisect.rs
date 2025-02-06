pub fn bisect_f<T: Fn(f64) -> f64>(a: f64, b: f64, f: &T) -> (f64, f64) {
    let m = (a + b) * 0.5;
    let (left, mid) = (f(a), f(m));
    if left * mid < 0.0 {
        return (a, m);
    }
    return (m, b);
}

pub fn bisect<T: Fn(f64) -> f64>(f: T, a: f64, b: f64, eps: f64) -> (f64, f64) {
    let res = (1..).scan((a, b), |acc, _| {
        let (x, y) = *acc;
        let new_interval = bisect_f(x, y, &f);
        *acc = new_interval;
        Some(new_interval)
    });
    res.take_while(|&(x, y)| (y - x).abs() > eps)
        .last()
        .unwrap_or((a, b))
}
