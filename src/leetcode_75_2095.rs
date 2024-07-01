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
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref()?.next.is_none() {
            return None;
        }
        let mut head = head;
        let mut fast = head.clone();
        let mut slow = &mut head;

        fast = fast?.next?.next;

        while fast.is_some() && fast.as_ref()?.next.is_some() {
            fast = fast?.next?.next;
            slow = &mut slow.as_mut()?.next;
        }
        if let Some(mut next_node) = slow.as_mut()?.next.take() {
            slow.as_mut()?.next = next_node.next.take();
        }
        head
    }
}
