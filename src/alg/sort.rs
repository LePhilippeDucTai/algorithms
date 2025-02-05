fn split(v: Vec<f64>, value: f64) -> (Vec<f64>, Vec<f64>) {
    let (left, right) = v.into_iter().partition(|&x| x < value);
    (left, right.split_at(1).1.to_vec())
}

pub fn quicksort(v: Vec<f64>) -> Vec<f64> {
    if v.len() <= 1 {
        return v;
    }
    let value = v[0];
    let (left, right) = split(v, value);
    let left_sorted = quicksort(left);
    let right_sorted = quicksort(right);
    [left_sorted, vec![value], right_sorted].concat()
}
