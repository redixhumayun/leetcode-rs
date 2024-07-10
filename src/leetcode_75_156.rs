// Definition for a binary tree node.
use crate::Solution;

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
    pub fn upside_down_binary_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn recurse(
            node: Option<Rc<RefCell<TreeNode>>>,
            root: &mut Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            match node {
                None => return None,
                Some(node) => {
                    let left = node.borrow_mut().left.take();
                    let right = node.borrow_mut().right.take();
                    if left.is_none() && right.is_none() {
                        if root.is_none() {
                            *root = Some(node.clone());
                        }
                        return Some(node);
                    }

                    let new_node = recurse(left, root);
                    node.borrow_mut().left = None;
                    node.borrow_mut().right = None;
                    if let Some(new_node) = new_node.as_ref() {
                        new_node.borrow_mut().left = right;
                        new_node.borrow_mut().right = Some(node.clone());
                    }
                    Some(node)
                }
            }
        }
        let mut new_root = None;
        recurse(root, &mut new_root);
        return new_root;
    }
}
