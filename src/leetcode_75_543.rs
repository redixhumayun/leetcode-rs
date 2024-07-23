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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn calculate_height(node: Option<&Rc<RefCell<TreeNode>>>, diameter: &mut i32) -> i32 {
            match node {
                None => 0,
                Some(current_node) => {
                    let left_height =
                        calculate_height(current_node.borrow().left.as_ref(), diameter);
                    let right_height =
                        calculate_height(current_node.borrow().right.as_ref(), diameter);
                    *diameter = std::cmp::max(*diameter, left_height + right_height);
                    1 + std::cmp::max(left_height, right_height)
                }
            }
        }

        let mut diameter = 0;
        calculate_height(root.as_ref(), &mut diameter);
        diameter
    }
}
