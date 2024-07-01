use crate::Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        //  find the middle node of the list
        let mut head = head;
        let mut fast = &head.clone();
        let mut slow = &mut head;
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
            slow = &mut slow.as_mut().unwrap().next;
        }

        //  starting with slow, reverse the second half of the list
        let mut prev: Option<Box<ListNode>> = None;
        while let Some(mut node) = slow.take() {
            let next = node.next.take();
            node.next = prev.take();
            prev = Some(node);
            *slow = next;
        }

        let mut max_pair_sum = 0;
        while head.is_some() && prev.is_some() {
            let sum = head.as_ref().unwrap().val + prev.as_ref().unwrap().val;
            max_pair_sum = std::cmp::max(max_pair_sum, sum);
            head = head.take().unwrap().next;
            prev = prev.take().unwrap().next;
        }
        max_pair_sum
    }
}
