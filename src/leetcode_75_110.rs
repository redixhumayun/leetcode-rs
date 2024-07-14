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
use std::rc::Rc;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn recurse(node: Option<Rc<RefCell<TreeNode>>>, depth: usize, result: &mut bool) -> usize {
            match node {
                None => return depth - 1,
                Some(current_node) => {
                    let left = recurse(current_node.borrow().left.clone(), depth + 1, result);
                    let right = recurse(current_node.borrow().right.clone(), depth + 1, result);
                    if left.abs_diff(right) > 1 {
                        *result = false;
                    }
                    return std::cmp::max(left, right);
                }
            }
        }
        let mut result = true;
        recurse(root, 0, &mut result);
        result
    }
}
