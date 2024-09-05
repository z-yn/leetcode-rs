pub struct Solution {}
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

// submission codes start

impl Solution {
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut dummy_head;
        let mut overflow = 0;
        while l1.is_some() || l2.is_some() {
            let v1 = if let Some(node) = l1 {
                l1 = node.next;
                node.val
            } else {
                0
            };
            let v2 = if let Some(node) = l2 {
                l2 = node.next;
                node.val
            } else {
                0
            };
            let sum = v1 + v2 + overflow;
            let v = if sum > 9 {
                overflow = 1;
                sum - 10
            } else {
                overflow = 0;
                sum
            };
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(v)));
            tail = &mut tail.as_mut().unwrap().next
        }
        if overflow == 1 {
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(1)))
        }
        dummy_head.unwrap().next
    }
}
// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        if vec.is_empty() {
            None
        } else {
            let mut r = Some(Box::from(ListNode::new(vec[0])));
            let mut b = &mut r;

            for i in 1..vec.len() {
                b.as_mut().unwrap().next = Some(Box::from(ListNode::new(vec[i])));
                b = &mut b.as_mut().unwrap().next
            }
            r
        }
    }
    #[test]
    fn test_1() {
        assert_eq!(to_list(vec![7, 0, 8]), Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])));
        assert_eq!(to_list(vec![8, 9, 9, 9, 0, 0, 0, 1]), Solution::add_two_numbers(to_list(vec![9, 9, 9, 9, 9, 9, 9]), to_list(vec![9, 9, 9, 9])));
        assert_eq!(to_list(vec![0, 0, 2]), Solution::add_two_numbers(to_list(vec![9, 9, 1]), to_list(vec![1])));
    }
}