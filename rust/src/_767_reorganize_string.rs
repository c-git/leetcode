//! Solution for https://leetcode.com/problems/reorganize-string
//! 767. Reorganize String
struct Letter {
    c: char,
    count: u16,
}

impl PartialOrd for Letter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.count.partial_cmp(&other.count)
    }
}

impl PartialEq for Letter {
    fn eq(&self, other: &Self) -> bool {
        self.c == other.c && self.count == other.count
    }
}
impl Ord for Letter {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.count.cmp(&other.count)
    }
}
impl Eq for Letter {}

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut char_counts = vec![0; 26];
        for c in s.as_bytes() {
            char_counts[(*c - b'a') as usize] += 1;
        }
        let mut remaining_chars = vec![];
        for (c, count) in char_counts.into_iter().enumerate() {
            if count > 0 {
                remaining_chars.push(Letter {
                    c: char::from_u32((b'a' + c as u8) as u32).unwrap(),
                    count,
                })
            }
        }
        remaining_chars.sort_unstable();
        let mut result = String::with_capacity(s.len());
        let mut last_seen = remaining_chars.last().unwrap().c;
        remaining_chars.last_mut().unwrap().count -= 1;
        remaining_chars.sort_unstable();
        result.push(last_seen);
        while !remaining_chars.is_empty() {
            if remaining_chars.last().unwrap().c != last_seen {
                let letter = remaining_chars.last_mut().unwrap();
                last_seen = letter.c;
                result.push(last_seen);
                if letter.count == 1 {
                    remaining_chars.pop();
                } else {
                    letter.count -= 1;
                }
            } else {
                if remaining_chars.len() < 2 {
                    return "".to_string();
                }
                let n = remaining_chars.len();
                let letter = remaining_chars.get_mut(n - 2).unwrap();
                last_seen = letter.c;
                result.push(last_seen);
                if letter.count == 1 {
                    remaining_chars.remove(n - 2);
                } else {
                    letter.count -= 1;
                }
            }
            remaining_chars.sort_unstable();
        }
        result
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("aab", "aba")]
    #[case("aaab", "")]
    fn case(#[case] s: String, #[case] expected: String) {
        let actual = Solution::reorganize_string(s);
        assert_eq!(actual, expected);
    }
}
