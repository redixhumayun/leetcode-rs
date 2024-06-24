pub struct Solution {}

impl Solution {
    fn is_divisible(prefix: &str, str1: &str, str2: &str) -> bool {
        if str1.len() % prefix.len() != 0 || str2.len() % prefix.len() != 0 {
            return false;
        }
        let str1_mul = str1.len() / prefix.len();
        let str2_mul = str2.len() / prefix.len();
        let repeated_1 = prefix.repeat(str1_mul);
        let repeated_2 = prefix.repeat(str2_mul);
        if repeated_1 == str1 && repeated_2 == str2 {
            return true;
        }
        return false;
    }

    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str1 == str2 {
            return str1;
        }
        let (shorter, longer) = {
            if str1.len() > str2.len() {
                (str2, str1)
            } else {
                (str1, str2)
            }
        };
        let mut index = shorter.len() - 1;
        while index >= 0 {
            let prefix_string = shorter[0..index + 1].to_string();
            println!("prefix {}", prefix_string);
            if Self::is_divisible(&prefix_string, &shorter, &longer) {
                return prefix_string;
            }
            if index == 0 {
                break;
            }
            index = index.saturating_sub(1);
        }
        return String::new();
    }
}
