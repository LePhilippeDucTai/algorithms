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


pub fn mergesorted(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut iter_a = a.into_iter().peekable();
    let mut iter_b = b.into_iter().peekable();
    std::iter::from_fn(|| match (iter_a.peek(), iter_b.peek()) {
        (Some(&v1), Some(&v2)) if v1 <= v2 => iter_a.next(),
        (Some(_), Some(_)) => iter_b.next(),
        (Some(_), None) => iter_a.next(),
        (None, Some(_)) => iter_b.next(),
        _ => None,
    })
    .collect()
}
