use std::collections::HashSet;

pub struct Solution {}
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.into_bytes();
        let mut exist = HashSet::new();
        let mut window_end = 0;
        let mut max_length = 0;

        let length = bytes.len();
        for window_start in 0..length {
            //window [window_start,window_end)
            if window_start != 0 {
                exist.remove(&bytes[window_start - 1]);
            }
            //max length
            while window_end < length && !exist.contains(&bytes[window_end]) {
                exist.insert(&bytes[window_end]);
                window_end += 1;
            }
            max_length = max_length.max(window_end - window_start);
        }
        max_length as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::length_of_longest_substring(" ".to_string()));
        assert_eq!(2, Solution::length_of_longest_substring("aab".to_string()));
        assert_eq!(3, Solution::length_of_longest_substring("abcabcbb".to_string()));
        assert_eq!(1, Solution::length_of_longest_substring("bbbbb".to_string()));
        assert_eq!(3, Solution::length_of_longest_substring("pwwkew".to_string()));
    }
}