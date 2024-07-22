// Definition for a binary tree node.
use crate::Solution;

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::i32;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn recurse(node: Option<&Rc<RefCell<TreeNode>>>, left: i64, right: i64) -> bool {
            match node {
                Some(current_node) => {
                    if current_node.borrow().val as i64 <= left {
                        return false;
                    }
                    if current_node.borrow().val as i64 >= right {
                        return false;
                    }
                    return recurse(
                        current_node.borrow().left.as_ref(),
                        left,
                        current_node.borrow().val as i64,
                    ) && recurse(
                        current_node.borrow().right.as_ref(),
                        current_node.borrow().val as i64,
                        right,
                    );
                }
                None => {
                    return true;
                }
            }
        }
        recurse(root.as_ref(), i64::MIN, i64::MAX)
    }
}
