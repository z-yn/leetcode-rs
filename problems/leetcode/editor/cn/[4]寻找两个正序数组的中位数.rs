//给定两个大小分别为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。请你找出并返回这两个正序数组的 中位数 。 
//
// 算法的时间复杂度应该为 O(log (m+n)) 。 
//
// 
//
// 示例 1： 
//
// 
//输入：nums1 = [1,3], nums2 = [2]
//输出：2.00000
//解释：合并数组 = [1,2,3] ，中位数 2
// 
//
// 示例 2： 
//
// 
//输入：nums1 = [1,2], nums2 = [3,4]
//输出：2.50000
//解释：合并数组 = [1,2,3,4] ，中位数 (2 + 3) / 2 = 2.5
// 
//
// 
//
// 
//
// 提示： 
//
// 
// nums1.length == m 
// nums2.length == n 
// 0 <= m <= 1000 
// 0 <= n <= 1000 
// 1 <= m + n <= 2000 
// -10⁶ <= nums1[i], nums2[i] <= 10⁶ 
// 
//
// Related Topics 数组 二分查找 分治 👍 7214 👎 0

//leetcode submit region begin(Prohibit modification and deletion)
use std::cmp::min;
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
//leetcode submit region end(Prohibit modification and deletion)
