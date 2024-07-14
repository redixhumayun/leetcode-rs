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
            match node {
                Some(current_node) => {
                    if current_node.borrow().val == p.as_ref().unwrap().borrow().val
                        || current_node.borrow().val == q.as_ref().unwrap().borrow().val
                    {
                        return Some(current_node);
                    }
                    let left = recurse(current_node.borrow().left.clone(), p.clone(), q.clone());
                    let right = recurse(current_node.borrow().right.clone(), p.clone(), q.clone());
                    match (left, right) {
                        (Some(_), Some(_)) => return Some(current_node),
                        (Some(l), None) => return Some(l),
                        (None, Some(r)) => return Some(r),
                        (None, None) => return None,
                    }
                }
                None => {
                    return None;
                }
            }
        }
        recurse(root, p, q)
    }
}
