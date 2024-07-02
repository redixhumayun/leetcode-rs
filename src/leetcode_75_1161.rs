use crate::Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
        queue.push_back(root);

        let mut max_sum = 0;
        while !queue.is_empty() {
            let level_size = queue.len();

            for i in 0..level_size {
                let mut level_sum = 0;
                if let Some(Some(node)) = queue.pop_front() {
                    level_sum += node.borrow().val;
                    if let Some(node) = node.borrow().left.clone() {
                        queue.push_back(Some(node));
                    }
                    if let Some(node) = node.borrow().right.clone() {
                        queue.push_back(Some(node));
                    }
                }
                max_sum = std::cmp::max(max_sum, level_sum);
            }
        }
        max_sum
    }
}
