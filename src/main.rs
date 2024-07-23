pub mod leetcode_75_543;

use std::{cell::RefCell, rc::Rc};

use leetcode_75_543::TreeNode;

pub struct Solution {}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    })));
    let left_root = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: None,
        right: None,
    })));
    left_root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: None,
        right: None,
    })));
    left_root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: None,
        right: None,
    })));
    root.as_ref().unwrap().borrow_mut().left = left_root;
    root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: None,
        right: None,
    })));
    let result = Solution::diameter_of_binary_tree(root);
    println!("result {}", result);
}
