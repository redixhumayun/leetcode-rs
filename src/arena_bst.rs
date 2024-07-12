#![allow(dead_code)]

use std::fmt::Display;

#[derive(Debug)]
enum BSTError {
    NodeNotFound,
    IndexExceededLimits,
}

impl Display for BSTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BSTError::NodeNotFound => write!(f, "Node not found"),
            BSTError::IndexExceededLimits => write!(f, "Index exceeded limits"),
        }
    }
}

#[derive(Debug)]
struct Node<T>
where
    T: PartialOrd + PartialEq + Clone + std::fmt::Debug,
{
    index: usize,
    left: Option<usize>,
    right: Option<usize>,
    parent: Option<usize>,
    value: T,
}

struct Arena<T>
where
    T: PartialOrd + PartialEq + Clone + std::fmt::Debug,
{
    tree: Vec<Node<T>>,
    count: usize,
    free_list: Vec<usize>,
}

impl<T> Arena<T>
where
    T: PartialOrd + PartialEq + Clone + std::fmt::Debug + Default,
{
    fn new() -> Self {
        Arena {
            tree: Vec::new(),
            count: 0,
            free_list: Vec::new(),
        }
    }

    fn get(&self, idx: usize) -> Result<&Node<T>, BSTError> {
        if idx >= self.tree.len() {
            return Err(BSTError::IndexExceededLimits);
        }

        self.tree.get(idx).ok_or_else(|| BSTError::NodeNotFound)
    }

    fn get_mut(&mut self, idx: usize) -> Result<&mut Node<T>, BSTError> {
        if idx >= self.tree.len() {
            return Err(BSTError::IndexExceededLimits);
        }

        self.tree.get_mut(idx).ok_or_else(|| BSTError::NodeNotFound)
    }

    fn insert(&mut self, val: T) -> Result<bool, BSTError> {
        if self.tree.len() == 0 {
            self.allocate_node(val, None);
            return Ok(true);
        }

        let mut parent_index: Option<usize> = None;
        let mut index: Option<usize> = Some(0);
        let mut is_left_child = true;

        while let Some(idx) = index {
            let current_node = self.get(idx)?;
            if current_node.value == val {
                return Ok(false); //  the data already exists
            }
            parent_index = Some(idx);
            if val < current_node.value {
                index = current_node.left;
                is_left_child = true;
            } else {
                index = current_node.right;
                is_left_child = false;
            }
        }

        if let Some(parent_index) = parent_index {
            let new_node_idx = self.allocate_node(val, Some(parent_index));
            if is_left_child {
                self.tree[parent_index].left = Some(new_node_idx);
            } else {
                self.tree[parent_index].right = Some(new_node_idx);
            }
        }
        Ok(true)
    }

    fn allocate_node(&mut self, val: T, parent: Option<usize>) -> usize {
        if let Some(free_index) = self.free_list.pop() {
            self.tree[free_index] = Node {
                index: free_index,
                left: None,
                right: None,
                parent,
                value: val,
            };
            free_index
        } else {
            let index = self.tree.len();
            self.count += 1;
            self.tree.push(Node {
                index,
                left: None,
                right: None,
                parent,
                value: val,
            });
            index
        }
    }

    fn in_order_traversal(&self, idx: Option<usize>, output: &mut Vec<T>) {
        if let Some(idx) = idx {
            if let Some(node) = self.tree.get(idx) {
                self.in_order_traversal(node.left, output);
                output.push(node.value.clone());
                self.in_order_traversal(node.right, output);
            }
        }
    }

    fn find(&self, val: &T) -> Result<&Node<T>, BSTError> {
        let mut index = Some(0);
        while let Some(i) = index {
            let node = self.get(i)?;
            if node.value == *val {
                return Ok(node);
            }
            if *val < node.value {
                index = node.left;
            } else {
                index = node.right;
            }
        }
        Err(BSTError::NodeNotFound)
    }

    ///  Find the minimum value in a subtree
    fn find_min(&self, idx: usize) -> Result<T, BSTError> {
        let mut node = self.tree.get(idx).unwrap();
        while let Some(left_idx) = node.left {
            node = self.get(left_idx)?;
        }
        Ok(node.value.clone())
    }

    /// Find the maximum value in a subtree  
    fn find_max(&self, idx: usize) -> Result<T, BSTError> {
        let mut node = self.get(idx).unwrap();
        while let Some(right_idx) = node.right {
            node = self.get(right_idx)?;
        }
        Ok(node.value.clone())
    }

    fn in_order_successor(&self, val: T) -> Result<T, BSTError> {
        let mut parent_index = None;
        let mut index = Some(0);
        while let Some(i) = index {
            let node = self.get(i)?;
            if node.value == val {
                break;
            }
            if val < node.value {
                parent_index = index;
                index = node.left;
            } else {
                index = node.right;
            }
        }

        match index {
            Some(idx) => {
                let node = self.get(idx)?;
                match node.right {
                    Some(right_idx) => {
                        return self.find_min(right_idx);
                    }
                    None => {
                        if let Some(parent_idx) = parent_index {
                            return Ok(self.get(parent_idx)?.value.clone());
                        }
                    }
                }
                return Err(BSTError::NodeNotFound);
            }
            None => return Err(BSTError::NodeNotFound),
        }
    }

    fn in_order_predecessor(&self, val: T) -> Result<T, BSTError> {
        let mut parent_index = None;
        let mut index = Some(0);
        while let Some(i) = index {
            let node = self.get(i)?;
            if node.value == val {
                break;
            }
            if val < node.value {
                index = node.left;
            }
            if val > node.value {
                parent_index = index;
                index = node.right;
            }
        }

        match index {
            Some(index) => {
                let node = self.get(index)?;
                match node.left {
                    Some(left_idx) => return self.find_max(left_idx),
                    None => {
                        if let Some(parent_index) = parent_index {
                            return Ok(self.get(parent_index)?.value.clone());
                        }
                    }
                }
            }
            None => return Err(BSTError::NodeNotFound),
        }
        return Err(BSTError::NodeNotFound);
    }

    fn delete(&mut self, val: T) -> Result<(), BSTError> {
        let (left, right, parent, index) = {
            let node = self.find(&val)?;
            let left = node.left;
            let right = node.right;
            let parent = node.parent;
            let index = node.index;
            (left, right, parent, index)
        };

        match (left, right) {
            (None, None) => {
                //  ideal case, remove the node
                if let Some(parent_idx) = parent {
                    let parent = self.get_mut(parent_idx)?;
                    if parent.left == Some(index) {
                        parent.left = None;
                    } else {
                        parent.right = None;
                    }
                } else {
                    //  this is the root node, set it to default for now
                    self.tree[index].value = T::default();
                }
                self.count -= 1;
                self.free_list.push(index);
            }
            (Some(left), None) => {
                if let Some(parent_idx) = parent {
                    //  the node has a left child
                    let parent = self.get_mut(parent_idx)?;
                    if parent.left == Some(index) {
                        parent.left = Some(left);
                    } else {
                        parent.right = Some(left);
                    }
                    let child = self.get_mut(left)?;
                    child.parent = Some(parent_idx);
                }
                self.count -= 1;
                self.free_list.push(index);
            }
            (None, Some(right)) => {
                if let Some(parent_idx) = parent {
                    //  the node has a right child
                    let parent = self.get_mut(parent_idx)?;
                    if parent.left == Some(index) {
                        parent.left = Some(right);
                    } else {
                        parent.right = Some(right);
                    }
                    let child = self.get_mut(right)?;
                    child.parent = Some(parent_idx);
                }
                self.count -= 1;
                self.free_list.push(index);
            }
            (Some(_), Some(_)) => {
                // both nodes are available, swap in order successors value and delete the successor node
                let successor = self.in_order_successor(val)?;
                let temp = successor.clone();
                self.delete(successor)?;
                self.tree[index].value = temp;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut arena: Arena<i32> = Arena::new();
        arena.insert(5).unwrap();
        arena.insert(2).unwrap();
        arena.insert(6).unwrap();
        arena.insert(1).unwrap();
        arena.insert(4).unwrap();
        arena.insert(3).unwrap();
        arena.insert(7).unwrap();
        assert_eq!(arena.count, 7);
        let mut output = Vec::new();
        arena.in_order_traversal(Some(0), &mut output);
        assert_eq!(output, vec![1, 2, 3, 4, 5, 6, 7]);

        assert_eq!(arena.find(&2).unwrap().value, 2);
        assert_eq!(arena.in_order_successor(2).unwrap(), 3);
        assert_eq!(arena.in_order_successor(4).unwrap(), 5);
        assert_eq!(arena.in_order_predecessor(2).unwrap(), 1);
        assert_eq!(arena.in_order_predecessor(3).unwrap(), 2);
        assert_eq!(arena.in_order_predecessor(7).unwrap(), 6);

        arena.delete(2).unwrap();
        output.clear();
        arena.in_order_traversal(Some(0), &mut output);
        assert_eq!(output, vec![1, 3, 4, 5, 6, 7]);
        assert_eq!(arena.count, 6);

        arena.insert(11).unwrap();
        assert_eq!(arena.count, 6);
        output.clear();
        arena.in_order_traversal(Some(0), &mut output);
        assert_eq!(output, vec![1, 3, 4, 5, 6, 7, 11]);
    }

    #[test]
    fn test_construct_tree() {
        let mut arena: Arena<i32> = Arena::new();
        arena.insert(5).unwrap();
        let l_child = arena.allocate_node(2, Some(0));
        let r_child = arena.allocate_node(6, Some(0));
        arena.get_mut(0).unwrap().left = Some(l_child);
        arena.get_mut(0).unwrap().right = Some(r_child);

        let ll_child = arena.allocate_node(1, Some(1));
        let lr_child = arena.allocate_node(4, Some(1));
        arena.get_mut(1).unwrap().left = Some(ll_child);
        arena.get_mut(1).unwrap().right = Some(lr_child);

        let mut output = Vec::new();
        arena.in_order_traversal(Some(0), &mut output);
        assert_eq!(output, vec![1, 2, 4, 5, 6]);
    }
}
