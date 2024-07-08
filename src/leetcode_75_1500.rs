use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap, HashSet},
};

type UserId = i32;
pub struct FileSharing {
    user_chunk_mapping: BTreeMap<UserId, HashSet<i32>>,
    next_id: i32,
    reusable_ids: BinaryHeap<Reverse<i32>>,
    chunks: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FileSharing {
    pub fn new(m: i32) -> Self {
        FileSharing {
            user_chunk_mapping: BTreeMap::new(),
            next_id: 1,
            reusable_ids: BinaryHeap::new(),
            chunks: m,
        }
    }

    pub fn join(&mut self, owned_chunks: Vec<i32>) -> i32 {
        let user_id = {
            match self.reusable_ids.pop() {
                Some(Reverse(user_id)) => user_id,
                None => {
                    let temp = self.next_id;
                    self.next_id += 1;
                    temp
                }
            }
        };

        let hash_set: HashSet<i32> = owned_chunks.iter().cloned().collect();
        self.user_chunk_mapping.insert(user_id, hash_set);
        user_id
    }

    pub fn leave(&mut self, user_id: i32) {
        self.user_chunk_mapping.remove(&user_id);
        self.reusable_ids.push(Reverse(user_id));
    }

    pub fn request(&self, user_id: i32, chunk_id: i32) -> Vec<i32> {
        let mut result = Vec::new();
        for (user, chunks) in &self.user_chunk_mapping {
            if chunks.contains(&chunk_id) {
                result.push(*user);
            }
        }
        result
    }
}
