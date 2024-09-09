//给定一个字符串 s ，请你找出其中不含有重复字符的 最长 子串 的长度。 
//
// 
//
// 示例 1: 
//
// 
//输入: s = "abcabcbb"
//输出: 3 
//解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
// 
//
// 示例 2: 
//
// 
//输入: s = "bbbbb"
//输出: 1
//解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
// 
//
// 示例 3: 
//
// 
//输入: s = "pwwkew"
//输出: 3
//解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
//     请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
// 
//
// 
//
// 提示： 
//
// 
// 0 <= s.length <= 5 * 10⁴ 
// s 由英文字母、数字、符号和空格组成 
// 
//
// Related Topics 哈希表 字符串 滑动窗口 👍 10306 👎 0

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashSet;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.into_bytes();
        let mut exist = HashSet::new();
        let mut window_end = 0;
        let mut max_length = 0;

        let length = bytes.len();
        for window_start in 0..length {
            if window_start != 0 {
                exist.remove(&bytes[window_start - 1]);
            }
            while window_end < length && !exist.contains(&bytes[window_end]) {
                exist.insert(&bytes[window_end]);
                window_end += 1;
            }
            max_length = max_length.max(window_end - window_start);
        }
        max_length as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)
