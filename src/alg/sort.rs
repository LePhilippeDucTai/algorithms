fn split<T: PartialOrd + Clone>(v: Vec<T>, value: &T) -> (Vec<T>, Vec<T>) {
    let (left, right) = v.into_iter().partition(|x| x < &value);
    (left, right.split_at(1).1.to_vec())
}

pub fn quicksort<T: PartialOrd + Clone>(v: Vec<T>) -> Vec<T> {
    if v.len() <= 1 {
        return v;
    }
    let value = v[0].clone();
    let (left, right) = split(v, &value);
    let left_sorted = quicksort(left);
    let right_sorted = quicksort(right);
    [left_sorted, vec![value], right_sorted].concat()
}
