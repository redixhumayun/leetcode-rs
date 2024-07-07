use crate::Solution;

/* https://leetcode.com/problems/non-overlapping-intervals/editorial/comments/1977299
Sorting intervals by start time instead of end time could lead to suboptimal results in certain scenarios when you're trying to solve the problem with a greedy algorithm.

Consider this example:

Interval 1: [1, 10]
Interval 2: [2, 3]
Interval 3: [4, 5]

If you sort by start time, you get the order: [1, 10], [2, 3], [4, 5]

If you start by taking the interval [1, 10] (as it starts first), you'll have to remove the other two intervals, which leads to removing 2 intervals.

However, if you sort by end time, you get the order: [2, 3], [4, 5], [1, 10]

Now, if you start by taking the interval that ends first ([2, 3]), you can then also take the next one that ends ([4, 5]), and finally, you have to remove [1, 10]. This leads to removing just 1 interval, which is the optimal solution.

So, sorting by start time may lead you to select an interval that overlaps with a maximum number of other intervals. On the other hand, sorting by end time ensures that you're always choosing the interval that leaves the most room for other intervals to follow, thus leading to the optimal solution in the context of this problem. */

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_by_key(|point| point[1]);
        println!("points sorted by end {:?}", points);
        points.sort_by_key(|point| point[0]);
        println!("points sorted by sta {:?}", points);
        let mut k = &points[0];
        let mut count = 1;
        for i in 1..points.len() {
            if points[i][0] <= k[1] {
                continue;
            }
            count += 1;
            k = &points[i];
        }
        count
    }
}
