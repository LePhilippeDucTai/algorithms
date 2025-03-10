use rand_distr::num_traits::Float;
use time_it_macro::time_it;

#[time_it]
pub fn bisect_f<T, F>(a: T, b: T, f: F) -> (T, T)
where
    F: Fn(T) -> T,
    T: Float,
{
    let m = (a + b) / T::from(2).unwrap();
    let (left, mid) = (f(a), f(m));
    if left * mid < T::zero() {
        return (a, m);
    }
    (m, b)
}

#[time_it]
pub fn bisect<T, F>(f: F, a: T, b: T, eps: T) -> (T, T)
where
    F: Fn(T) -> T,
    T: Float,
{
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bisect_f() {
        let f = |x| x;
        assert_eq!(bisect_f(-1., 1., f), (0., 1.));
        assert_eq!(bisect_f(-2., 1., f), (-0.5, 1.));
    }

    #[test]
    fn test_bisect() {
        let f = |x| x;
        let result = bisect(f, -2., 1., 1e-6);
        assert!(result.0.abs() < 1e-6);
        assert!(result.1.abs() < 1e-6);
    }
}
