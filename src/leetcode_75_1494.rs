use std::collections::{HashMap, VecDeque};

use crate::Solution;

impl Solution {
    pub fn min_number_of_semesters(n: i32, relations: Vec<Vec<i32>>, k: i32) -> i32 {
        fn top_sort(adj_list: &HashMap<i32, Vec<i32>>, indegree: &mut Vec<i32>, k: i32) -> i32 {
            let mut queue: VecDeque<i32> = VecDeque::new();
            //  possible to start up to k courses which have no pre-reqs
            for i in 1..indegree.len() {
                if indegree[i] == 0 {
                    queue.push_back(i as i32);
                }
            }

            let mut semesters = 0;
            while !queue.is_empty() {
                semesters += 1;
                for _ in 0..queue.len().min(k.try_into().unwrap()) {
                    if let Some(course) = queue.pop_front() {
                        if let Some(neighbour_courses) = adj_list.get(&course) {
                            for &neighbour_course in neighbour_courses {
                                indegree[neighbour_course as usize] -= 1;
                                if indegree[neighbour_course as usize] == 0 {
                                    queue.push_back(neighbour_course);
                                }
                            }
                        }
                    }
                }
            }
            semesters
        }

        //  build the adjacency list
        let mut adj_list: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut indegree = vec![0; (n + 1) as usize];
        for relation in relations {
            let pre_req = relation[0];
            let course = relation[1];
            adj_list
                .entry(pre_req)
                .and_modify(|e| e.push(course))
                .or_insert(vec![course]);
            indegree[course as usize] += 1;
        }
        top_sort(&adj_list, &mut indegree, k)
    }
}
