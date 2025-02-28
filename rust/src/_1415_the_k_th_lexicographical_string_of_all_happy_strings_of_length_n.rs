//! Solution for https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n
//! 1415. The k-th Lexicographical String of All Happy Strings of Length n

impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let n = n as usize;
        let chars: Vec<char> = ('a'..='c').collect();

        let mut curr: Vec<usize> = (0..n).map(|i| i % 2).collect();
        let mut pos = curr.len() - 1;
        for _ in 1..k {
            loop {
                curr[pos] += 1;
                if pos > 0 && curr[pos - 1] == curr[pos] {
                    curr[pos] += 1;
                }
                if curr[pos] < chars.len() {
                    // Reset the rest
                    for i in (pos + 1)..n {
                        curr[i] = if i > 0 {
                            // Set to 'a' if previous is not 'a' otherwise 'b'
                            if curr[i - 1] == 0 {
                                1
                            } else {
                                0
                            }
                        } else {
                            // At start cannot increment to 'a' so set to 'a'
                            0
                        };
                    }
                    pos = curr.len() - 1;
                    break;
                } else if pos > 0 {
                    pos -= 1;
                } else {
                    return "".to_string();
                }
            }
        }
        curr.into_iter().map(|i| chars[i]).collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(1, 3, "c")]
    #[case(1, 4, "")]
    #[case(3, 9, "cab")]
    #[case(5, 15, "acbca")]
    fn case(#[case] n: i32, #[case] k: i32, #[case] expected: String) {
        let actual = Solution::get_happy_string(n, k);
        assert_eq!(actual, expected);
    }
}
