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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut preorder_index = 0;
        fn helper(
            preorder_index: &mut usize,
            preorder: Vec<i32>,
            inorder: Vec<i32>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if *preorder_index >= preorder.len() {
                panic!("malformed tree")
            }
            if inorder.len() == 0 {
                return None;
            }
            if inorder.len() == 1 {
                *preorder_index += 1;
                return Some(Rc::new(RefCell::new(TreeNode::new(inorder[0]))));
            }
            let current_root = preorder[*preorder_index];
            let inorder_index = inorder
                .iter()
                .position(|elem| *elem == current_root)
                .unwrap();
            let left_tree = inorder[..inorder_index].to_vec();
            let right_tree = inorder[inorder_index + 1..].to_vec();
            *preorder_index += 1;
            let left_node = helper(preorder_index, preorder.clone(), left_tree);
            let right_node = helper(preorder_index, preorder, right_tree);
            let current_node = TreeNode::new(current_root);
            Some(Rc::new(RefCell::new(TreeNode {
                val: current_node.val,
                left: left_node,
                right: right_node,
            })))
        }
        helper(&mut preorder_index, preorder, inorder)
    }
}
