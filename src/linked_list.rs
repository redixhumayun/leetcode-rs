type NodeLink = Option<usize>;

#[derive(Debug, Clone)]
pub struct Node {
    val: i32,
    next: NodeLink,
}

pub struct LinkedList {
    arena: Vec<Node>,
    capacity: usize,
    head: NodeLink,
    tail: NodeLink,
    free_list: Vec<usize>,
}

impl LinkedList {
    pub fn new() -> Self {
        Self {
            arena: Vec::new(),
            capacity: 0,
            head: None,
            tail: None,
            free_list: Vec::new(),
        }
    }

    fn allocate_node(&mut self, val: i32) -> usize {
        let index = self.arena.len();
        let new_node = Node { val, next: None };
        self.capacity += 1;
        self.arena.push(new_node);
        index
    }

    fn deallocate_node(&mut self, index: usize) {
        self.free_list.push(index);
        self.capacity -= 1;
    }

    fn update_node(&mut self, index: usize, node: Node) {
        if index >= self.capacity {
            panic!("out of bounds");
        }
        self.arena[index] = node;
    }

    fn get(&self, index: usize) -> Node {
        if index >= self.capacity {
            panic!("out of bounds");
        }
        self.arena.get(index).unwrap().clone()
    }

    fn get_mut(&mut self, index: usize) -> &mut Node {
        if index >= self.capacity {
            panic!("out of bounds");
        }
        self.arena.get_mut(index).unwrap()
    }

    pub fn prepend(&mut self, val: i32) {
        match self.head.take() {
            Some(old_head) => {
                let new_head_index = self.allocate_node(val);
                let new_head = self.get_mut(new_head_index);
                new_head.next = Some(old_head);
                self.head = Some(new_head_index);
            }
            None => {
                let new_node_index = self.allocate_node(val);
                self.head = Some(new_node_index);
                self.tail = Some(new_node_index);
            }
        }
    }

    pub fn append(&mut self, val: i32) {
        match self.tail.take() {
            Some(old_tail_index) => {
                let new_tail_index = self.allocate_node(val);
                let old_tail = self.get_mut(old_tail_index);
                old_tail.next = Some(new_tail_index);
                self.tail = Some(new_tail_index);
            }
            None => {
                let new_node_index = self.allocate_node(val);
                self.head = Some(new_node_index);
                self.tail = Some(new_node_index);
            }
        }
    }

    pub fn remove_middle(&mut self) {
        let mut slow = self.head.clone();
        let mut fast = self.head.clone();
        fast = self.get(*fast.as_ref().unwrap()).next;
        fast = self.get(*fast.as_ref().unwrap()).next;

        while fast.is_some() && self.get(*fast.as_ref().unwrap()).next.is_some() {
            fast = self.get(*fast.as_ref().unwrap()).next;
            fast = self.get(*fast.as_ref().unwrap()).next;
            slow = self.get(*slow.as_ref().unwrap()).next;
        }
        let mut current_slow = self.get(*slow.as_ref().unwrap());
        let next_slow_node = self.get(*slow.as_ref().unwrap()).next;
        if let Some(next_slow) = next_slow_node {
            current_slow.next = self.get(next_slow).next;
        }
        self.update_node(*slow.as_ref().unwrap(), current_slow);
        self.deallocate_node(*next_slow_node.as_ref().unwrap());
    }

    pub fn iter(&self) -> Iter {
        Iter {
            current: self.head,
            arena: self.arena.clone(),
        }
    }
}

pub struct Iter {
    current: NodeLink,
    arena: Vec<Node>,
}

impl Iterator for Iter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(index) => {
                let node = self.arena.get(index).unwrap().clone();
                let value = node.val;
                self.current = node.next;
                Some(value)
            }
            None => None,
        }
    }
}
