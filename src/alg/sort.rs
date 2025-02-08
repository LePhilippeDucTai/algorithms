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

fn mergesorted(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut iter_a = a.into_iter().peekable();
    let mut iter_b = b.into_iter().peekable();
    let mut res = vec![];
    while let (Some(x), Some(y)) = (iter_a.peek(), iter_b.peek()) {
        if x < y {
            res.push(iter_a.next());
        } else {
            res.push(iter_b.next());
        }
    }
    // if iter_a.peek().is_none() {
    //     return res.iter().chain(iter_b).collect();
    // }
    vec![]
}
