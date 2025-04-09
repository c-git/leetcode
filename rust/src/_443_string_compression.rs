//! Solution for https://leetcode.com/problems/string-compression
//! 443. String Compression

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut last_char = *chars.first().unwrap();
        let mut curr_count = 1;
        let mut next_write_index = 0;
        for i in 1..chars.len() {
            if last_char == chars[i] {
                curr_count += 1;
            } else {
                chars[next_write_index] = last_char;
                next_write_index += 1;
                if curr_count > 1 {
                    for digit in curr_count.to_string().chars() {
                        chars[next_write_index] = digit;
                        next_write_index += 1;
                    }
                }
                last_char = chars[i];
                curr_count = 1;
            }
        }
        chars[next_write_index] = last_char;
        next_write_index += 1;
        if curr_count > 1 {
            for digit in curr_count.to_string().chars() {
                chars[next_write_index] = digit;
                next_write_index += 1;
            }
        }

        next_write_index as i32
    }
}
