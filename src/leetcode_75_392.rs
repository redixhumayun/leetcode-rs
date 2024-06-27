use crate::Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();
        let mut left = 0;
        let mut right = 0;
        while right < t_chars.len() {
            if left < s_chars.len() && s_chars[left] == t_chars[right] {
                left += 1;
            }
            right += 1;
        }
        left == s_chars.len()
    }
}
