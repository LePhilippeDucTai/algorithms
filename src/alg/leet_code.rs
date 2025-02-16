use std::collections::HashMap;

use itertools::Itertools;

pub fn two_sums(nums: Vec<i32>, target: i32) -> (usize, usize) {
    let enums = nums.iter().enumerate();
    let inverse_image: HashMap<i32, usize> = enums.clone().map(|(i, &x)| (target - x, i)).collect();
    let result = enums
        .filter(|(_, &x)| inverse_image.contains_key(&x))
        .map(|(i, x)| (i, inverse_image.get(x).unwrap().to_owned()))
        .filter(|(i, j)| i != j)
        .next()
        .unwrap();
    result
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let (i, j) = two_sums(nums, target);
    let (i_32, j_32) = ((i as i32), (j as i32));
    vec![i_32, j_32]
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let result = match (list1, list2) {
        (None, None) => None,
        (None, Some(v)) => Some(v),
        (Some(v), None) => Some(v),
        (Some(list_node_1), Some(list_node_2)) => {
            if list_node_1.val < list_node_2.val {
                let mut res = ListNode::new(list_node_1.val);
                res.next = merge_two_lists(list_node_1.next, Some(list_node_2));
                Some(Box::new(res))
            } else {
                let mut res = ListNode::new(list_node_2.val);
                res.next = merge_two_lists(Some(list_node_1), list_node_2.next);
                Some(Box::new(res))
            }
        }
    };
    return result;
}

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    lists
        .into_iter()
        .tree_reduce(|acc, x| merge_two_lists(acc, x))
        .flatten()
}

fn sqrt(x: u32) -> u32 {
    0
}
