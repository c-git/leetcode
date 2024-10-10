//! Solution for https://leetcode.com/problems/add-binary
//! 67. Add Binary

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut carry = false;
        let mut result_buf = vec![];
        let mut a = a.chars();
        let mut b = b.chars();
        loop {
            match (a.next_back(), b.next_back()) {
                (None, None) => {
                    if carry {
                        result_buf.push(true);
                    }
                    break;
                }
                (None, Some(bit)) | (Some(bit), None) => {
                    let is_one = bit != '0';
                    match (is_one, carry) {
                        (false, false) | (true, true) => result_buf.push(false),
                        (true, false) | (false, true) => {
                            result_buf.push(true);
                            carry = false;
                        }
                    }
                }
                (Some(bit_a), Some(bit_b)) => {
                    let (is_one_a, is_one_b) = (bit_a != '0', bit_b != '0');
                    match (is_one_a, is_one_b, carry) {
                        (true, true, true) => result_buf.push(true),
                        (true, true, false) => {
                            result_buf.push(false);
                            carry = true;
                        }
                        (true, false, true) | (false, true, true) => result_buf.push(false),
                        (true, false, false) | (false, true, false) => result_buf.push(true),
                        (false, false, false) => result_buf.push(false),
                        (false, false, true) => {
                            result_buf.push(true);
                            carry = false;
                        }
                    }
                }
            }
        }
        result_buf
            .into_iter()
            .rev()
            .map(|bit| if bit { "1" } else { "0" })
            .collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("11", "1", "100")]
    #[case("1010", "1011", "10101")]
    fn case(#[case] a: String, #[case] b: String, #[case] expected: String) {
        let actual = Solution::add_binary(a, b);
        assert_eq!(actual, expected);
    }
}
