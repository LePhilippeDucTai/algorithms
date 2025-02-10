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

pub fn mergesorted<T: PartialOrd + Copy>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
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

pub fn mergesort<T: PartialOrd + Copy>(v: Vec<T>) -> Vec<T> {
    let n = v.len();
    if n <= 1 {
        return v;
    }
    let mid = n / 2;
    let (left, right) = v.split_at(mid);
    mergesorted(mergesort(left.to_vec()), mergesort(right.to_vec()))
}

pub fn linearsearch(v: &Vec<i32>, value: i32) -> usize {
    let n = v.len();
    if n == 0 {
        return 0;
    }
    if value <= v[0] {
        return 0;
    }
    (1..n)
        .filter(|&i| (v[i - 1] < value) & (value <= v[i]))
        .next()
        .unwrap_or(n)
}
pub fn searchsorted_f(v: &Vec<i32>, value: i32) -> usize {
    let n = v.len();
    let n_iter_max = n.ilog2() + 2;
    let solution = (0..n_iter_max).fold((0 as usize, n), |acc, _| {
        let (i, j) = acc;
        let m = (i + j) / 2;
        if (v[i] < value) & (value <= v[m]) {
            return (i, m);
        }
        (m, j)
    });
    solution.1
}

pub fn searchsorted(v: &Vec<i32>, value: i32) -> usize {
    if v.len() <= 8 {
        return linearsearch(&v, value);
    }
    searchsorted_f(&v, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_searchsorted_f() {
        let v = vec![1, 5, 10, 17, 19, 20, 23, 24, 25, 29, 30];
        let val = 15;
        assert_eq!(searchsorted_f(&v, val), 3);
        assert_eq!(linearsearch(&v, val), 3);
    }

    #[test]
    fn test_search() {
        let v = vec![1, 5, 10, 17, 19, 20, 23, 24, 25, 29, 30];
        let val = 15;
        assert_eq!(searchsorted_f(&v, val), linearsearch(&v, val))
    }

    #[test]
    fn test_mergesort_empty() {
        let v: Vec<i32> = vec![];
        let sorted = mergesort(v);
        assert_eq!(sorted, vec![]);
    }

    #[test]
    fn test_mergesort_single_element() {
        let v = vec![1];
        let sorted = mergesort(v);
        assert_eq!(sorted, vec![1]);
    }

    #[test]
    fn test_mergesort_sorted() {
        let v = vec![1, 2, 3, 4, 5];
        let sorted = mergesort(v);
        assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_mergesort_unsorted() {
        let v = vec![5, 3, 1, 4, 2];
        let sorted = mergesort(v);
        assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_mergesort_duplicates() {
        let v = vec![3, 1, 2, 3, 1];
        let sorted = mergesort(v);
        assert_eq!(sorted, vec![1, 1, 2, 3, 3]);
    }

    #[test]
    fn test_linearsearch_empty() {
        let v: Vec<i32> = vec![];
        let index = linearsearch(&v, 3);
        assert_eq!(index, 0);
    }

    #[test]
    fn test_linearsearch_first_element() {
        let v = vec![1, 2, 3, 4, 5];
        let index = linearsearch(&v, 1);
        assert_eq!(index, 0);
    }

    #[test]
    fn test_linearsearch_middle_element() {
        let v = vec![1, 2, 3, 4, 5];
        let index = linearsearch(&v, 3);
        assert_eq!(index, 2);
    }

    #[test]
    fn test_linearsearch_last_element() {
        let v = vec![1, 2, 3, 4, 5];
        let index = linearsearch(&v, 5);
        assert_eq!(index, 4);
        assert_eq!(searchsorted_f(&v, 5), 4);
    }

    #[test]
    fn test_linearsearch_not_found() {
        let v = vec![1, 2, 3, 4, 5];
        let index = linearsearch(&v, 6);
        assert_eq!(index, 5);
    }
    #[test]
    fn test_linearsearch_less_than_min() {
        let v = vec![1, 2, 3, 4, 5];
        let index = linearsearch(&v, -1);
        assert_eq!(index, 0);
    }

    #[test]
    fn test_linearsearch_1() {
        let v = vec![0, 1, 2, 3, 19, 20, 31];
        let index = linearsearch(&v, 32);
        assert_eq!(index, 7);
    }
}

// pub fn searchsorted(v: Vec<i32>, value: i32) -> usize {
//     if v.len() <= 10 {
//         return linearsearch(v, value);
//     }
//     0
// match (v.len(), v) {
//     (0, _) => vec![value],
//     (1, s) => {
//         if s[0] < value {
//             vec![s[0], value]
//         }
//     }
//     (_, _) {
//         let n = v.len();
//         let m = n / 2;
//         let (a, mid) = (v[0], v[mid]);
//     }
// }
// }
