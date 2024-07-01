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
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut odd_list: Option<Box<ListNode>> = None;
        let mut even_list: Option<Box<ListNode>> = None;
        let mut odd_tail = &mut odd_list;
        let mut even_tail = &mut even_list;
        let mut counter = 0;

        while let Some(mut node) = head {
            head = node.next.take();
            if counter % 2 == 0 {
                //  odd node
                *odd_tail = Some(node);
                odd_tail = &mut odd_tail.as_mut()?.next;
            } else {
                *even_tail = Some(node);
                even_tail = &mut even_tail.as_mut()?.next;
            }
            counter += 1;
        }

        *odd_tail = even_list;

        odd_list
    }
}
