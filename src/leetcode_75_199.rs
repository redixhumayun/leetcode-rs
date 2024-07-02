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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return Vec::new();
        }

        let mut queue = VecDeque::new();
        queue.push_back(root);

        let mut last_nodes = Vec::new();
        while !queue.is_empty() {
            let level_size = queue.len();
            for i in 0..level_size {
                if let Some(Some(node)) = queue.pop_front() {
                    if i == level_size - 1 {
                        last_nodes.push(node.borrow().val);
                    }
                    if let Some(left) = node.borrow().left.clone() {
                        queue.push_back(Some(left));
                    }
                    if let Some(right) = node.borrow().right.clone() {
                        queue.push_back(Some(right));
                    }
                }
            }
        }
        last_nodes
    }
}
