//! Solution for https://leetcode.com/problems/backspace-string-compare
//! 844. Backspace String Compare

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        // Based on second solution in editorial
        // Go back one character at a time in each string and ensure each character is the same
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut pos_s = s.len() as i32; // Starts to the right of the last letter as the first step is to move back one
        let mut pos_t = t.len() as i32;
        loop {
            // Find "next char" in each final string
            step_back_to_next_kept_char(&mut pos_s, s);
            step_back_to_next_kept_char(&mut pos_t, t);
            let is_s_finished = pos_s < 0;
            let is_t_finished = pos_t < 0;
            match (is_s_finished, is_t_finished) {
                (true, true) => {
                    // No more characters to compare, they must be the same since they never did not match
                    return true;
                }
                (false, false) => {
                    if s[pos_s as usize] != t[pos_t as usize] {
                        // This character stays but it's not the same
                        return false;
                    }
                }
                (true, false) | (false, true) => {
                    // Only one string finished
                    return false;
                }
            }
        }
    }
}

/// Goes back until it meets a character that is kept or moves pos before first character
fn step_back_to_next_kept_char(pos: &mut i32, chars: &[u8]) {
    let mut backspace = 0;
    loop {
        *pos -= 1;
        if *pos < 0 {
            break;
        }
        if chars[*pos as usize] == b'#' {
            // Record need to skip one more
            backspace += 1;
        } else if backspace > 0 {
            // Skip this one
            backspace -= 1;
        } else {
            // This character gets to stay
            break;
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("ab#c", "ad#c", true)]
    #[case("ab##", "c#d#", true)]
    #[case("a#c", "b", false)]
    fn case(#[case] s: String, #[case] t: String, #[case] expected: bool) {
        let actual = Solution::backspace_compare(s, t);
        assert_eq!(actual, expected);
    }
}
