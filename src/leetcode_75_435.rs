use crate::Solution;

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_by_key(|x| x[1]);
        let mut start = intervals[0][0];
        let mut end = intervals[0][1];

        let mut count = 0;
        for i in 1..intervals.len() {
            if intervals[i][0] < end || intervals[i][1] < end {
                count += 1;
                continue;
            }
            start = intervals[i][0];
            end = intervals[i][1];
        }
        count
    }
}
