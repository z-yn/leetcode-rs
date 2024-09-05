pub struct Solution {}

// submission codes start
impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut bytes=Vec::new();
        for x in s.as_bytes() {
            if x.is_ascii_digit() {
                if !bytes.is_empty() { bytes.pop(); }
            } else {
                bytes.push(*x)
            }
        }
        String::from_utf8_lossy(&bytes).parse().unwrap()
    }

}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("abc", Solution::clear_digits("abc".parse().unwrap()));
        assert_eq!("", Solution::clear_digits("cb34".parse().unwrap()));
    }
}