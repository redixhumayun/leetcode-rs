use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use crate::Solution;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        fn djikstra(
            arrival_time: &mut Vec<i32>,
            source: i32,
            adj_list: &HashMap<i32, Vec<(i32, usize)>>,
        ) {
            let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
            heap.push(Reverse((0, source as usize)));
            arrival_time[source as usize] = 0;

            while let Some(Reverse((arr_time, node))) = heap.pop() {
                //  find all neighbours of the node
                if arr_time > arrival_time[node as usize] {
                    continue; //  shortest path to this node already found
                }
                match adj_list.get(&(node as i32)) {
                    Some(neighbours) => {
                        for (time, neighbour) in neighbours {
                            if arrival_time[*neighbour] > arr_time + time {
                                arrival_time[*neighbour] = arr_time + time;
                                heap.push(Reverse((arrival_time[*neighbour], *neighbour)));
                            }
                        }
                    }
                    None => (),
                }
            }
        }

        //  build the adjacency list
        let mut hash_map: HashMap<i32, Vec<(i32, usize)>> = HashMap::new();
        for time in times {
            hash_map
                .entry(time[0])
                .and_modify(|e| e.push((time[2], time[1] as usize)))
                .or_insert(vec![(time[2], time[1] as usize)]);
        }
        let mut arrival_time: Vec<i32> = vec![i32::MAX; (n + 1) as usize];
        djikstra(&mut arrival_time, k, &hash_map);
        let mut max_arrival_time = i32::MIN;
        for index in 1..arrival_time.len() {
            max_arrival_time = std::cmp::max(arrival_time[index], max_arrival_time);
            if arrival_time[index] == i32::MAX {
                return -1;
            }
        }
        max_arrival_time
    }
}
