#![allow(dead_code)]

use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
enum BSTError {
    NodeNotFound,
}

impl Display for BSTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BSTError::NodeNotFound => write!(f, "Node not found"),
        }
    }
}

impl Error for BSTError {}

type NodeLink = Option<Box<Node>>;

#[derive(Debug, Clone, PartialEq)]
struct Node {
    data: i32,
    left: NodeLink,
    right: NodeLink,
}

struct Tree {
    root: NodeLink,
}

impl Tree {
    fn new(val: i32) -> Self {
        let node = Node {
            data: val,
            left: None,
            right: None,
        };
        Tree {
            root: Some(Box::new(node)),
        }
    }

    fn insert(&mut self, data: i32) -> bool {
        let mut root = &mut self.root;
        while let Some(ref mut node) = root {
            if data < node.as_ref().data {
                root = &mut node.left;
            } else {
                root = &mut node.right;
            }
        }
        *root = Some(Box::new(Node {
            data,
            left: None,
            right: None,
        }));
        true
    }

    fn find(&self, data: i32) -> NodeLink {
        let mut current = &self.root;
        while let Some(node) = current {
            if node.data == data {
                return Some(node.clone());
            }
            if data < node.data {
                current = &node.left;
            } else {
                current = &node.right;
            }
        }
        None
    }

    //  returns (parent, node, L or R)
    fn find_with_parent(&self, data: i32) -> Result<(NodeLink, NodeLink, String), BSTError> {
        let mut parent: NodeLink = None;
        let mut node = &self.root;
        let mut string = "".to_string();
        while let Some(current) = node {
            if current.data == data {
                return Ok((parent, Some(current.clone()), string));
            }
            if data < current.data {
                parent = Some(current.clone());
                node = &current.left;
                string = "L".to_string();
            } else {
                parent = Some(current.clone());
                node = &current.right;
                string = "R".to_string();
            }
        }

        return Err(BSTError::NodeNotFound);
    }

    /// If the node has a right child, then it is the smallest node in the right subtree
    /// Otherwise, find the closest parent in whose left subtree this node lives
    fn in_order_successor(&self, data: i32) -> Result<i32, BSTError> {
        let node = self.find(data).ok_or_else(|| BSTError::NodeNotFound)?;

        match node.right {
            Some(node) => Ok(Self::find_min(node).data),
            None => {
                let mut parent: NodeLink = None;
                let mut root = &self.root;
                while let Some(node) = root {
                    if node.data == data {
                        return Ok(parent.ok_or_else(|| BSTError::NodeNotFound)?.data);
                    }
                    if data < node.data {
                        parent = Some(node.clone());
                        root = &node.left;
                    } else {
                        root = &node.right;
                    }
                }
                return Err(BSTError::NodeNotFound);
            }
        }
    }

    /// If the node has a left child, then it is the largest node in the left subtree
    /// Otherwise, find the closest parent in whose right subtree this node lives
    fn in_order_predecessor(&self, data: i32) -> Result<i32, BSTError> {
        let node = self.find(data).ok_or_else(|| BSTError::NodeNotFound)?;
        match node.left {
            Some(node) => Ok(self.find_max(node)),
            None => {
                let mut parent: NodeLink = None;
                let mut root = &self.root;
                while let Some(node) = root {
                    if data == node.data {
                        return Ok(parent.ok_or_else(|| BSTError::NodeNotFound)?.data);
                    }
                    if data < node.data {
                        root = &node.left;
                    } else {
                        parent = Some(node.clone());
                        root = &node.right;
                    }
                }
                return Err(BSTError::NodeNotFound);
            }
        }
    }

    fn find_min(node: Box<Node>) -> Box<Node> {
        let mut current = node;
        while let Some(left) = current.left {
            current = left;
        }
        current
    }

    fn find_max(&self, node: Box<Node>) -> i32 {
        let mut current = node;
        while let Some(right) = current.right {
            current = right;
        }
        current.data
    }

    fn delete(&mut self, data: i32) -> Result<(), BSTError> {
        self.root = Self::delete_node(self.root.take(), data);
        Ok(())
    }

    fn delete_node(node: NodeLink, val: i32) -> NodeLink {
        match node {
            Some(mut node) => {
                if val < node.data {
                    node.left = Self::delete_node(node.left, val);
                    Some(node)
                } else if val > node.data {
                    node.right = Self::delete_node(node.right, val);
                    Some(node)
                } else {
                    if node.left.is_none() {
                        return node.right;
                    }
                    if node.right.is_none() {
                        return node.left;
                    }

                    let min = Self::find_min(node.right.take().unwrap());
                    node.data = min.data;
                    Self::delete_node(node.right.clone(), node.data); //  the clone is okay here because node.right is set to None because of the take() 2 lines above
                    Some(node)
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bst() {
        let mut tree = Tree::new(5);
        tree.insert(2);
        tree.insert(6);
        tree.insert(1);
        tree.insert(4);
        tree.insert(3);
        tree.insert(7);

        assert_eq!(tree.find(3).unwrap().data, 3);
        assert_eq!(tree.find(8), None);
        assert_eq!(tree.find(7).unwrap().data, 7);

        assert_eq!(tree.in_order_successor(3).unwrap(), 4);
        assert_eq!(tree.in_order_successor(4).unwrap(), 5);
        assert_eq!(tree.in_order_successor(5).unwrap(), 6);
        assert_eq!(tree.in_order_successor(9), Err(BSTError::NodeNotFound));

        assert_eq!(tree.in_order_predecessor(3).unwrap(), 2);
        assert_eq!(tree.in_order_predecessor(2).unwrap(), 1);
        assert_eq!(tree.in_order_predecessor(5).unwrap(), 4);
        assert_eq!(tree.in_order_predecessor(4).unwrap(), 3);
        assert_eq!(tree.in_order_predecessor(7).unwrap(), 6);

        tree.delete(3).unwrap();
        assert_eq!(tree.find(3), None);
        assert_eq!(tree.in_order_successor(2).unwrap(), 4);
    }
}
