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

pub enum TreeNodeDirection {
    Default,
    Left,
    Right,
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recurse(node: Option<Rc<RefCell<TreeNode>>>, prev_direction: TreeNodeDirection) -> i32 {
            match node {
                Some(n) => {
                    let n = n.borrow();
                    match prev_direction {
                        TreeNodeDirection::Default => {
                            let left = recurse(n.left.clone(), TreeNodeDirection::Left);
                            let right = recurse(n.right.clone(), TreeNodeDirection::Right);
                            left.max(right)
                        }
                        TreeNodeDirection::Left => {
                            let right = recurse(n.right.clone(), TreeNodeDirection::Right);
                            1 + right
                        }
                        TreeNodeDirection::Right => {
                            let left = recurse(n.left.clone(), TreeNodeDirection::Left);
                            1 + left
                        }
                    }
                }
                None => 0,
            }
        }
        recurse(root, TreeNodeDirection::Default)
    }
}
