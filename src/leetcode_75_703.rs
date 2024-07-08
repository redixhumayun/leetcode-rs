pub struct KthLargest {
    k: i32,
    nums: Vec<i32>,
    top: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut nums = nums;
        nums.sort();
        let start_index = nums.len() - (k as usize - 1);
        let extracted = &nums[start_index..];
        let top = nums[nums.len() - k as usize];
        KthLargest {
            k,
            nums: extracted.to_vec(),
            top,
        }
    }

    pub fn add(&mut self, val: i32) -> i32 {
        if val >= self.nums[0] {
            let index = self.binary_search(val);
            self.nums.insert(index, val);
            let removed = self.nums.remove(0);
            self.top = removed;
            return self.top;
        }
        self.top
    }

    fn binary_search(&self, val: i32) -> usize {
        let mut left = 0;
        let mut right = self.nums.len();
        while left < right {
            let mid = left + (right - left) / 2;
            let mid_value = self.nums[mid];
            if val <= mid_value {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}
