use std::cmp::min;
use std::fmt::Display;

pub struct Solution {}
// submission codes start

impl Solution {
    fn get_n_th_number(num1: &Vec<i32>, s1: usize, num2: &Vec<i32>, s2: usize, k: usize) -> i32 {
        // println!("find top {} in  num1={:?} num2={:?}", k, &num1[s1..], &num2[s2..]);
        let len1 = num1.len() - s1;
        let len2 = num2.len() - s2;
        if len1 == 0 {
            return num2[s2 + k - 1];
        }
        if len2 == 0 {
            return num1[s1 + k - 1];
        }
        if k == 1 {
            return min(num1[s1], num2[s2]);
        }
        //从开始数k/2个，判断需要丢弃的部分
        let middle1 = s1 + min(k / 2, len1) - 1;
        let middle2 = s2 + min(k / 2, len2) - 1;
        if num1[middle1] > num2[middle2] {
            Self::get_n_th_number(num1, s1, num2, middle2 + 1, k - (middle2 + 1 - s2))
        } else {
            Self::get_n_th_number(num2, s2, num1, middle1 + 1, k - (middle1 + 1 - s1))
        }
    }
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (nums1, nums2) = (&nums1, &nums2);
        let total_len = nums1.len() + nums2.len();
        let half_len = (total_len + 1) / 2;
        let half_value = Self::get_n_th_number(nums1, 0, nums2, 0, half_len) as f64;
        if total_len % 2 == 1 {
            half_value
        } else {
            (Self::get_n_th_number(nums1, 0, nums2, 0, half_len + 1) as f64 + half_value) * 0.5
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(3.5, Solution::find_median_sorted_arrays(vec![1, 2, 3], vec![4, 5, 6]));

        assert_eq!(3.0, Solution::find_median_sorted_arrays(vec![1, 2, 3], vec![4, 5]));
        assert_eq!(2.0, Solution::find_median_sorted_arrays(vec![1, 2, 3], vec![1, 2]));
    }
}