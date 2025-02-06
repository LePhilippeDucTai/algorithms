use std::collections::HashMap;

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
