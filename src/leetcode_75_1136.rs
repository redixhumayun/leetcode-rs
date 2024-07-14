use std::collections::{HashMap, VecDeque};

use crate::Solution;

type AdjList = HashMap<usize, Vec<usize>>;

impl Solution {
    pub fn minimum_semesters(n: i32, relations: Vec<Vec<i32>>) -> i32 {
        fn top_sort(adj_list: &AdjList, indegree: &mut Vec<i32>, n: usize) -> i32 {
            let mut num_of_sems = 0;
            let mut courses_taken = 0;
            let mut queue = VecDeque::new();
            for i in 1..indegree.len() {
                if indegree[i] == 0 {
                    queue.push_back(i);
                    courses_taken += 1;
                }
            }

            if queue.len() == 0 {
                return -1;
            }

            while !queue.is_empty() {
                num_of_sems += 1;
                for _ in 0..queue.len() {
                    if let Some(current_course) = queue.pop_front() {
                        if let Some(future_courses) = adj_list.get(&current_course) {
                            for future_course in future_courses {
                                indegree[*future_course] -= 1;
                                if indegree[*future_course] == 0 {
                                    courses_taken += 1;
                                    queue.push_back(*future_course);
                                }
                            }
                        }
                    }
                }
            }
            if courses_taken < n {
                return -1;
            }
            num_of_sems
        }

        let mut adj_list: AdjList = HashMap::new();
        let mut indegree = vec![0; (n + 1) as usize];
        for relation in relations {
            let pre_req = relation[0];
            let req = relation[1];
            adj_list
                .entry(pre_req as usize)
                .and_modify(|e| e.push(req as usize))
                .or_insert(vec![req as usize]);
            indegree[req as usize] += 1;
        }
        top_sort(&adj_list, &mut indegree, n as usize)
    }
}
