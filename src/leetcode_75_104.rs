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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recurse(node: Option<Rc<RefCell<TreeNode>>>, count: i32) -> i32 {
            match node {
                Some(n) => {
                    let n = n.borrow();
                    let left_depth = recurse(n.left.clone(), count + 1);
                    let right_depth = recurse(n.right.clone(), count + 1);
                    std::cmp::max(left_depth, right_depth)
                }
                None => count,
            }
        }
        recurse(root, 0)
    }
}
