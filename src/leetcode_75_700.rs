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
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(n) => {
                let n_borrowed = n.borrow();
                if n_borrowed.val == val {
                    return Some(n.clone());
                } else if n_borrowed.val < val {
                    return Self::search_bst(n_borrowed.right.clone(), val);
                }
                return Self::search_bst(n_borrowed.left.clone(), val);
            }
            None => None,
        }
    }
}
