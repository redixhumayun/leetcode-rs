use std::fmt::Display;

type NodeIndex = usize;
type NodeLink = Option<NodeIndex>;

#[derive(Debug)]
pub enum DoublyLinkedListError {
    IndexOutOfBounds,
    NodeNotFound,
}

impl Display for DoublyLinkedListError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DoublyLinkedListError::IndexOutOfBounds => {
                write!(f, "index out of bounds")
            }
            DoublyLinkedListError::NodeNotFound => {
                write!(f, "node not found")
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    val: i32,
    next: NodeLink,
    prev: NodeLink,
    index: NodeIndex,
}

pub struct DoublyLinkedList {
    arena: Vec<Node>,
    head: NodeLink,
    tail: NodeLink,
    capacity: usize,
    free_list: Vec<NodeIndex>,
}

impl DoublyLinkedList {
    pub fn new() -> Self {
        DoublyLinkedList {
            arena: Vec::new(),
            head: None,
            tail: None,
            capacity: 0,
            free_list: Vec::new(),
        }
    }

    fn allocate_node(&mut self, val: i32) -> Node {
        let node_index = self.capacity;
        let new_node = Node {
            val,
            next: None,
            prev: None,
            index: node_index,
        };
        self.arena.push(new_node.clone());
        self.capacity += 1;
        new_node
    }

    fn get(&self, index: NodeIndex) -> Result<Node, DoublyLinkedListError> {
        self.arena
            .get(index)
            .ok_or_else(|| DoublyLinkedListError::IndexOutOfBounds)
            .cloned()
    }

    fn get_mut(&mut self, index: NodeIndex) -> Result<&mut Node, DoublyLinkedListError> {
        if index >= self.capacity {
            return Err(DoublyLinkedListError::IndexOutOfBounds);
        }
        self.arena
            .get_mut(index)
            .ok_or_else(|| DoublyLinkedListError::IndexOutOfBounds)
    }

    fn update(&mut self, index: NodeIndex, new_node: Node) -> Result<(), DoublyLinkedListError> {
        if index >= self.capacity {
            return Err(DoublyLinkedListError::IndexOutOfBounds);
        }
        self.arena[index] = new_node;
        Ok(())
    }

    pub fn append(&mut self, val: i32) -> Result<(), DoublyLinkedListError> {
        let mut new_node = self.allocate_node(val);
        match self.tail.take() {
            Some(old_tail_index) => {
                new_node.prev = Some(old_tail_index);
                let old_tail_node = self.get_mut(old_tail_index)?;
                old_tail_node.next = Some(new_node.index);
                self.tail = Some(new_node.index);
                self.update(new_node.index, new_node)?;
            }
            None => {
                self.head = Some(new_node.index);
                self.tail = Some(new_node.index);
            }
        }
        Ok(())
    }

    pub fn prepend(&mut self, val: i32) -> Result<(), DoublyLinkedListError> {
        let mut new_node = self.allocate_node(val);
        match self.head.take() {
            Some(old_head_index) => {
                new_node.next = Some(old_head_index);
                let old_head = self.get_mut(old_head_index)?;
                old_head.prev = Some(new_node.index);
                self.head = Some(new_node.index);
                self.update(new_node.index, new_node)?;
            }
            None => {
                self.head = Some(new_node.index);
                self.tail = Some(new_node.index);
            }
        }
        Ok(())
    }

    pub fn find(&self, val: i32) -> Result<Node, DoublyLinkedListError> {
        let mut current_index = &self.head;
        let mut current_node = self.get(current_index.as_ref().unwrap().clone())?;
        while current_node.val != val && current_node.next.is_some() {
            current_index = &current_node.next;
            current_node = self.get(current_index.as_ref().unwrap().clone())?;
        }
        if current_node.val != val {
            return Err(DoublyLinkedListError::NodeNotFound);
        }

        Ok(current_node)
    }

    pub fn move_to_end(&mut self, mut node: Node) -> Result<(), DoublyLinkedListError> {
        if let Some(prev_index) = node.prev {
            let mut prev_node = self.get(prev_index)?;
            if let Some(next_index) = node.next {
                let mut next_node = self.get(next_index)?;
                prev_node.next = Some(next_node.index);
                next_node.prev = Some(prev_node.index);
                self.update(prev_index, prev_node)?;
                self.update(next_index, next_node)?;
            } else {
                //  this is the tail node, so do nothing
                return Ok(());
            }
        } else {
            //  this is the head node
            if let Some(next_index) = node.next {
                self.head = Some(next_index);
            } else {
                //  this is a solitary node
                return Ok(());
            }
        }

        //  now move the node to the back of the linked list
        match self.tail.take() {
            Some(old_tail_index) => {
                node.prev = Some(old_tail_index);
                let old_tail_node = self.get_mut(old_tail_index)?;
                old_tail_node.next = Some(node.index);
                node.next = None;
                self.tail = Some(node.index);
                self.update(node.index, node)?;
            }
            None => {
                panic!("solitary node already covered under configuration");
            }
        }
        Ok(())
    }

    pub fn iter(&self) -> Iter {
        Iter {
            current: self.head.clone(),
            arena: &self.arena,
        }
    }
}

pub struct Iter<'a> {
    current: NodeLink,
    arena: &'a [Node],
}

impl<'a> Iterator for Iter<'a> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(node_index) => {
                let value = self.arena.get(node_index).unwrap().val;
                self.current = self.arena.get(node_index).unwrap().next;
                Some(value)
            }
            None => None,
        }
    }
}
