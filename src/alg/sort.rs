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
    if v.len() <= 10 {
        return linearsearch(&v, value);
    }
    searchsorted_f(&v, value)
}

fn insertion(acc: &mut Vec<i32>, value: i32) -> Vec<i32> {
    let index = searchsorted(acc, value);
    acc.insert(index, value);
    acc.to_vec()
}

pub fn insertionsort(v: &Vec<i32>) -> Vec<i32> {
    if v.is_empty() {
        return v.clone();
    }
    let result: Vec<i32> = v.iter().fold(vec![], |mut acc: Vec<i32>, value: &i32| {
        insertion(&mut acc, *value)
    });
    return result;
}

fn heap_it(mut v: &mut Vec<i32>, i: usize) -> &mut Vec<i32> {
    let (left, right) = (2 * i + 1, 2 * i + 2);
    let n = v.len();
    let mut largest = i;
    if left < n {
        if v[left] > v[largest] {
            largest = left;
        }
    }
    if right < n {
        if v[right] > v[largest] {
            largest = right;
        }
    }
    if largest != i {
        v.swap(largest, i);
        v = heap_it(v, largest);
    }
    v
}

pub fn heapify(v: &mut Vec<i32>) -> Vec<i32> {
    let n = v.len();
    let mut v_cop = v.clone();
    for i in (0..(n / 2)).rev() {
        v_cop = heap_it(&mut v_cop, i).to_vec();
    }
    v_cop
}

pub fn heapsort(v: &Vec<i32>) -> Vec<i32> {
    let mut v_copy = v.clone();
    let mut result: Vec<i32> = vec![];
    while !v_copy.is_empty() {
        let last = v_copy.len() - 1;
        v_copy = heapify(&mut v_copy);
        v_copy.swap(0, last);
        result.push(v_copy.pop().unwrap());
    }
    result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heapify() {
        let mut v = vec![1, 10, 41, 2, 10, 20, 1, 21, 4];
        v = heapify(&mut v);
        println!("{v:?}")
    }

    #[test]
    fn test_heapsort() {
        let v = vec![1, 10, 41, 2, 10, 20, 1, 21, 4];
        let result = heapsort(&v);
        println!("{result:?}")
    }

    #[test]
    fn test_searchsorted_f() {
        let v = vec![1, 5, 10, 17, 19, 20, 23, 24, 25, 29, 30];
        let val = 15;
        assert_eq!(searchsorted_f(&v, val), 3);
        assert_eq!(linearsearch(&v, val), 3);
    }

    #[test]
    fn test_insertionsort() {
        let v = vec![1, 5, 10, 17, 19, 20, 23, 24, 25, 29, 30];
        assert_eq!(insertionsort(&v), v)
    }

    #[test]
    fn test_insertionsort_unsorted() {
        let v = vec![5, 3, 1, 4, 2];
        let sorted = insertionsort(&v);
        assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
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
