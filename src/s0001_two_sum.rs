
pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i,&v) in nums.iter().enumerate() {
            if let Some(&i1) = map.get(&(target-v)) {
                return vec![i1,i as i32];
            } else {
                map.insert(v,i as i32);
            }
        }
       vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}