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
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut leaves_1: Vec<i32> = Vec::new();
        let mut leaves_2: Vec<i32> = Vec::new();

        fn recurse(node: Option<Rc<RefCell<TreeNode>>>, leaves: &mut Vec<i32>) {
            match node {
                Some(n) => {
                    //  check if leaf node
                    if n.borrow().left.is_none() && n.borrow().right.is_none() {
                        leaves.push(n.borrow().val);
                    }
                    recurse(n.borrow().left.clone(), leaves);
                    recurse(n.borrow().right.clone(), leaves);
                }
                None => return,
            }
        }
        recurse(root1, &mut leaves_1);
        recurse(root2, &mut leaves_2);
        leaves_1 == leaves_2
    }
}
