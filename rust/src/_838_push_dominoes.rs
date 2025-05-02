//! Solution for https://leetcode.com/problems/push-dominoes
//! 838. Push Dominoes

use std::collections::VecDeque;

impl Solution {
    /// Based on https://www.youtube.com/watch?v=evUFsOb_iLY
    pub fn push_dominoes(dominoes: String) -> String {
        let mut dominoes = dominoes.as_bytes().to_vec();
        let mut queue: VecDeque<_> = dominoes
            .iter()
            .enumerate()
            .filter_map(|(i, &c)| if c != b'.' { Some(i) } else { None })
            .collect();
        while let Some(idx) = queue.pop_front() {
            match dominoes[idx] {
                b'L' => {
                    // Knock over left domino if possible
                    if idx == 0 {
                        continue;
                    }
                    if dominoes[idx - 1] == b'.' {
                        dominoes[idx - 1] = b'L';
                        queue.push_back(idx - 1);
                    }
                }
                b'R' => {
                    // Knock over to the right if possible
                    if idx + 1 < dominoes.len() && dominoes[idx + 1] == b'.' {
                        if idx + 2 < dominoes.len() && dominoes[idx + 2] == b'L' {
                            // The center domino stays up and we skip the next left domino
                            let next = queue.pop_front();
                            debug_assert_eq!(dominoes[next.unwrap()], b'L');
                            continue;
                        }
                        dominoes[idx + 1] = b'R';
                        queue.push_back(idx + 1);
                    }
                }
                _ => unreachable!("Only L and R should be added"),
            }
        }
        std::str::from_utf8(&dominoes).unwrap().to_string()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("RR.L", "RR.L")]
    #[case(".L.R...LR..L..", "LL.RR.LLRRLL..")]
    fn case(#[case] dominoes: String, #[case] expected: String) {
        let actual = Solution::push_dominoes(dominoes);
        assert_eq!(actual, expected);
    }
}
