use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct HeapWrapper {
    cost: i32,
    destination: i32,
    steps: usize,
}

use crate::Solution;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        fn djikstra(
            arrival_cost: &mut Vec<i32>,
            source: i32,
            k: i32,
            adj_list: &HashMap<i32, Vec<(i32, i32)>>,
        ) {
            arrival_cost[source as usize] = 0;
            let mut heap: BinaryHeap<Reverse<HeapWrapper>> = BinaryHeap::new();
            heap.push(Reverse(HeapWrapper {
                cost: 0,
                destination: source,
                steps: 0,
            }));

            while let Some(Reverse(heap_elem)) = heap.pop() {
                let HeapWrapper {
                    cost,
                    destination,
                    steps,
                } = heap_elem;
                if cost > arrival_cost[destination as usize] {
                    continue; //  the cost of arriving at this node is greater than what I have
                }

                if steps == (k + 1) as usize {
                    continue; //  cannot go past k stops
                }

                if let Some(neighbours) = adj_list.get(&destination) {
                    for (cost_to_neighbour, neighbour) in neighbours {
                        if cost + cost_to_neighbour > arrival_cost[*neighbour as usize] {
                            continue; //  the cost to go to the neighbour is greater than what is there right now
                        }

                        let new_cost = cost + cost_to_neighbour;
                        arrival_cost[*neighbour as usize] = new_cost;
                        heap.push(Reverse(HeapWrapper {
                            cost: new_cost,
                            destination: *neighbour,
                            steps: steps + 1,
                        }));
                    }
                }
            }
        }

        //  build the adjacency matrix
        let mut adj_list: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
        for flight in flights {
            adj_list
                .entry(flight[0])
                .and_modify(|entry| entry.push((flight[2], flight[1])))
                .or_insert(vec![(flight[2], flight[1])]);
        }
        let mut arrival_cost: Vec<i32> = vec![i32::MAX; n as usize];
        djikstra(&mut arrival_cost, src, k, &adj_list);
        if arrival_cost[dst as usize] == i32::MAX {
            return -1;
        }
        arrival_cost[dst as usize]
    }
}
