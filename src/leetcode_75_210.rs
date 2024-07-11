use std::collections::{HashMap, VecDeque};

use crate::Solution;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        //  this is a variation of bfs where nodes with indegree of 0 are added to the queue
        fn top_sort(adj_list: &HashMap<i32, Vec<i32>>, indegree: &mut Vec<i32>) -> Vec<i32> {
            let mut queue: VecDeque<i32> = VecDeque::new(); //  the deque will hold current course
            let mut course_order = Vec::new();
            for (index, degree) in indegree.iter().enumerate() {
                if *degree == 0 {
                    queue.push_back(index as i32);
                }
            }
            //  no course to start with
            if queue.is_empty() {
                return Vec::new();
            }

            while let Some(course) = queue.pop_front() {
                course_order.push(course);
                if let Some(neighbour_courses) = adj_list.get(&course) {
                    for neighbour_course in neighbour_courses {
                        indegree[*neighbour_course as usize] -= 1;
                        if indegree[*neighbour_course as usize] == 0 {
                            queue.push_back(*neighbour_course);
                        }
                    }
                }
            }
            course_order
        }

        let mut adj_list: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut indegree = vec![0; num_courses as usize];
        for prereq in prerequisites {
            let first = prereq[1];
            let second = prereq[0];
            adj_list
                .entry(first)
                .and_modify(|e| e.push(second))
                .or_insert(vec![second]);
            indegree[second as usize] += 1;
        }
        let course_list = top_sort(&adj_list, &mut indegree);
        if course_list.len() != num_courses as usize {
            return Vec::new();
        }
        return course_list;
    }
}

/*
n = 4, preqrequisites = [[1,0],[2,0],[3,1],[3,2]]
0 -> 1 -> 2 -> 3 (1 and 2 can be interchanged)

adj_list -> {0: [1,2], 1: [3], 2: [3]}
indegree = [0, 1, 1, 2]
queue -> {0} -> pop 0 and new indegree [0, 0, 0, 2]
queue -> {1,2} -> pop 1 and new indegree [0, 0, 0, 1]
queue -> {2} -> pop 2 and new indegree [0, 0, 0, 0]
queue -> {3} -> pop 3 and done

Runtime

building adj list -> O(v+e) because vertices and edges visited once
top sort -> O(v+e) because each node is visited once and each edge is only traversed once

Space -> O(v+e) for adj list, indegree -> O(v) where v is number of courses
*/
