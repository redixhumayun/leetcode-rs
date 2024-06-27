use crate::Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut counter = 0;
        let mut prev_char = ' ';
        let mut write_index = 0;
        for i in 0..chars.len() {
            if chars[i] != prev_char {
                if prev_char != ' ' {
                    chars[write_index] = prev_char;
                    write_index += 1;
                    if counter > 1 {
                        let counter_as_chars: Vec<char> = counter.to_string().chars().collect();
                        for c in counter_as_chars {
                            chars[write_index] = c;
                            write_index += 1;
                        }
                    }
                }
                counter = 1;
                prev_char = chars[i];
                continue;
            }
            counter += 1;
        }

        //  handle the last characters
        chars[write_index] = prev_char;
        write_index += 1;
        if counter > 1 {
            let counter_as_chars: Vec<char> = counter.to_string().chars().collect();
            for c in counter_as_chars {
                chars[write_index] = c;
                write_index += 1;
            }
        }

        //  remove the remaining characters
        chars.truncate(write_index);
        println!("the chars {:?}", chars);
        (write_index) as i32
    }
}
