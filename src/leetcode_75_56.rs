use crate::Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort(); //  sort by start ts

        let mut merged_intervals = Vec::new();
        for interval in intervals {
            match merged_intervals.last() {
                None => merged_intervals.push(interval),
                Some(last_interval) => {
                    if interval[0] <= last_interval[1] {
                        let new_interval = [
                            std::cmp::min(last_interval[0], interval[0]),
                            std::cmp::max(last_interval[1], interval[1]),
                        ]
                        .to_vec();
                        merged_intervals.pop();
                        merged_intervals.push(new_interval);
                    } else {
                        merged_intervals.push(interval);
                    }
                }
            }
        }
        merged_intervals
    }
}
