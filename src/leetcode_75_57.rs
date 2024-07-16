use crate::Solution;

impl Solution {
    fn binary_search(intervals: &Vec<Vec<i32>>, new_interval: &Vec<i32>) -> usize {
        let mut low = 0;
        let mut high = intervals.len();
        while low < high {
            let mid = low + (high - low) / 2; //  avoid overflow
            let mid_interval = &intervals[mid];
            if new_interval[0] < mid_interval[0] {
                high = mid;
            } else if new_interval[0] > mid_interval[0] {
                low = mid + 1;
            } else {
                return low;
            }
        }
        low
    }

    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        //  insert new interval
        let insertion_index = Solution::binary_search(&intervals, &new_interval);
        let mut intervals = intervals;
        intervals.insert(insertion_index, new_interval);

        //  merge overlapping intervals
        let mut merged_intervals = Vec::new();
        let mut prev_interval: Vec<i32> = Vec::new();
        for interval in intervals {
            if prev_interval.is_empty() {
                prev_interval = interval.clone();
                merged_intervals.push(interval);
                continue;
            }

            if interval[0] <= prev_interval[1] {
                let popped_interval = merged_intervals.pop().unwrap();
                let new_interval = [
                    std::cmp::min(popped_interval[0], interval[0]),
                    std::cmp::max(popped_interval[1], interval[1]),
                ]
                .to_vec();
                prev_interval = new_interval.clone();
                merged_intervals.push(new_interval);
            } else {
                prev_interval = interval.clone();
                merged_intervals.push(interval);
            }
        }
        merged_intervals
    }
}

/*
[[1,2],[3,5],[6,7],[8,10],[12,16]], [4,8]
[[1,2],[3,5],[4,8],[6,7],[8,10],[12,16]]

[[1,2]]
[[1,2], [3,5]]
[[1,2], [3,8]]
[[1,2], [3,8]]
[[1,2], [3,10]]
[[1,2], [3,10], [12,16]]
*/
