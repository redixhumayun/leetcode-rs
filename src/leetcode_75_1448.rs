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
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let current_max = -100_000;
        fn recurse(node: Option<Rc<RefCell<TreeNode>>>, max: i32) -> i32 {
            match node {
                Some(n) => {
                    let n = n.borrow();
                    if n.val >= max {
                        return recurse(n.left.clone(), n.val)
                            + recurse(n.right.clone(), n.val)
                            + 1;
                    } else {
                        return recurse(n.left.clone(), max) + recurse(n.right.clone(), max);
                    }
                }
                None => return 0,
            }
        }
        recurse(root, current_max)
    }
}
