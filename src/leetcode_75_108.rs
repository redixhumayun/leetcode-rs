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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn recurse(arr: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            match arr.len() {
                0 => None,
                1 => Some(Rc::new(RefCell::new(TreeNode {
                    val: arr[0],
                    left: None,
                    right: None,
                }))),
                _ => {
                    let mid = arr.len() / 2;
                    let left = recurse(&arr[..mid]);
                    let right = recurse(&arr[mid + 1..]);
                    Some(Rc::new(RefCell::new(TreeNode {
                        val: arr[mid],
                        left,
                        right,
                    })))
                }
            }
        }

        recurse(&nums)
    }
}
