use std::{cell::RefCell, rc::Rc};

use leetcode_75_1161::TreeNode;

// pub mod leetcode_75_1431;
// pub mod leetcode_75_2;
// pub mod leetcode_75_605;
// use leetcode_75_605::Solution;
pub mod leetcode_75_1161;

pub struct Solution {}

fn main() {
    let tree: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode::new(10))));

    let mut left_subtree = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    left_subtree.as_mut().unwrap().borrow_mut().left =
        Some(Rc::new(RefCell::new(TreeNode::new(3))));
    left_subtree.as_mut().unwrap().borrow_mut().right =
        Some(Rc::new(RefCell::new(TreeNode::new(2))));
    left_subtree
        .as_mut()
        .unwrap()
        .borrow_mut()
        .left
        .as_ref()
        .unwrap()
        .borrow_mut()
        .left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    left_subtree
        .as_mut()
        .unwrap()
        .borrow_mut()
        .left
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right = Some(Rc::new(RefCell::new(TreeNode::new(-2))));
    tree.as_ref().unwrap().borrow_mut().left = left_subtree;

    let mut right_subtree = Some(Rc::new(RefCell::new(TreeNode::new(-3))));
    right_subtree.as_mut().unwrap().borrow_mut().right =
        Some(Rc::new(RefCell::new(TreeNode::new(11))));
    tree.as_ref().unwrap().borrow_mut().right = right_subtree;

    let result = Solution::max_level_sum(tree.clone());
    println!("The result {:?}", result);
}
