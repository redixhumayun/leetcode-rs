use crate::Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        let vowels = "aeiouAEIOU";
        let mut left = 0 as usize;
        let mut right = chars.len().saturating_sub(1);
        while left < right {
            if !vowels.contains(chars[left]) {
                left += 1;
            }
            if !vowels.contains(chars[right]) {
                right -= 1;
            }
            if vowels.contains(chars[left]) && vowels.contains(chars[right]) {
                chars.swap(left, right);
                left += 1;
                right = right.saturating_sub(1);
            }
        }
        chars.into_iter().collect()
    }
}
