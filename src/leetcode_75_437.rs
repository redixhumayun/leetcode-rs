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
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut hash_map: HashMap<i32, i32> = HashMap::new();
        hash_map.insert(0, 1);
        fn recurse(
            node: Option<Rc<RefCell<TreeNode>>>,
            curr_sum: i32,
            target_sum: i32,
            count: i32,
            hash_map: &mut HashMap<i32, i32>,
        ) -> i32 {
            match node {
                Some(n) => {
                    let sum = curr_sum + n.borrow().val;
                    let mut new_count = count;
                    let diff = sum - target_sum;
                    if hash_map.contains_key(&diff) {
                        new_count += hash_map.get(&diff).unwrap();
                    }
                    return recurse(n.borrow().left, sum, target_sum, new_count, hash_map)
                        + recurse(n.borrow().right, sum, target_sum, new_count, hash_map);
                }
                None => return count,
            }
        }
        recurse(root, 0, target_sum, 0, &mut hash_map)
    }
}
