//! Solution for https://leetcode.com/problems/minimum-domino-rotations-for-equal-row
//! 1007. Minimum Domino Rotations For Equal Row

impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let mut freq_top = [0; 7];
        let mut freq_bottom = [0; 7];
        let mut prev_double_value = None;

        for i in 0..tops.len() {
            let is_double = tops[i] == bottoms[i];
            match (is_double, prev_double_value) {
                (true, None) => {
                    // We must use this value
                    let double_value = tops[i];
                    if freq_top[double_value as usize] + freq_bottom[double_value as usize] != i {
                        // We must use double_value but each of top and bottom don't always have the value
                        return -1;
                    }
                    prev_double_value = Some(tops[i]);
                }
                (true, Some(double_value)) => {
                    if double_value != tops[i] {
                        // We must use double_value but found a different number
                        return -1;
                    }
                }
                (false, Some(double_value)) => {
                    // Ensure at least one of the new values is double_value
                    if double_value != tops[i] && double_value != bottoms[i] {
                        // We'll never be able to make it as there is no way to rotate this domino to match
                        return -1;
                    }
                    freq_top[tops[i] as usize] += 1;
                    freq_bottom[bottoms[i] as usize] += 1;
                }
                (false, None) => {
                    // All dominos are not doubles so far just keep tracking frequencies
                    freq_top[tops[i] as usize] += 1;
                    freq_bottom[bottoms[i] as usize] += 1;
                }
            }
        }

        if let Some(double_value) = prev_double_value {
            // If we got here only values in freq for double_value are those that were not double so just return the smaller of the two values
            freq_top[double_value as usize].min(freq_bottom[double_value as usize]) as i32
        } else {
            let mut result = i32::MAX;
            for i in 1..=6 {
                if freq_top[i] + freq_bottom[i] == tops.len() {
                    result = result.min(freq_top[i].min(freq_bottom[i]) as i32);
                }
            }
            if result == i32::MAX {
                -1
            } else {
                result
            }
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
    #[case(vec![2,1,2,4,2,2], vec![5,2,6,2,3,2], 2)]
    #[case(vec![3,5,1,2,3], vec![3,6,3,3,4], -1)]
    fn case(#[case] tops: Vec<i32>, #[case] bottoms: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::min_domino_rotations(tops, bottoms);
        assert_eq!(actual, expected);
    }
}
