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

use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn recurse(
            node: Option<Rc<RefCell<TreeNode>>>,
            p: Option<Rc<RefCell<TreeNode>>>,
            q: Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if node.is_none() {
                return None;
            }
            if node == p || node == q {
                return node;
            }

            let left = recurse(node.unwrap().borrow().left, p, q);
            let right = recurse(node.unwrap().borrow().right, p, q);

            match (left, right) {
                (Some(_), Some(_)) => node,
                (Some(tree), None) => Some(tree),
                (None, Some(tree)) => Some(tree),
                (None, None) => None,
            }
        }
        recurse(root, p, q);
        None
    }
}
