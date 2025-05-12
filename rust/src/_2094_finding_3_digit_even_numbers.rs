//! Solution for https://leetcode.com/problems/finding-3-digit-even-numbers
//! 2094. Finding 3-Digit Even Numbers

impl Solution {
    /// From editorial
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        let freq_counts = digits.into_iter().fold([0; 10], |mut freq, num| {
            freq[num as usize] += 1;
            freq
        });

        for first_digit in 1..=9 {
            if freq_counts[first_digit] == 0 {
                continue;
            }
            for second_digit in 0..=9 {
                if (second_digit == first_digit && freq_counts[second_digit] < 2)
                    || freq_counts[second_digit] < 1
                {
                    continue;
                }

                for last_digit in (0..=8).step_by(2) {
                    if freq_counts[last_digit] == 0 {
                        continue;
                    }
                    let mut add_to_result = || {
                        result.push((first_digit * 100 + second_digit * 10 + last_digit) as i32);
                    };
                    match (
                        first_digit == second_digit,
                        first_digit == last_digit,
                        second_digit == last_digit,
                    ) {
                        (true, true, true) => {
                            if freq_counts[first_digit] >= 3 {
                                add_to_result();
                            }
                        }
                        (true, false, false) => add_to_result(), // Already checked above
                        (false, true, false) => {
                            if freq_counts[first_digit] > 1 {
                                add_to_result()
                            }
                        }
                        (false, false, true) => {
                            if freq_counts[last_digit] > 1 {
                                add_to_result()
                            }
                        }
                        (false, false, false) => add_to_result(),
                        _ => unreachable!("This should be impossible but we got here with {first_digit} {second_digit} {last_digit}"),
                    }
                }
            }
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
    #[case(vec![2,1,3,0], vec![102,120,130,132,210,230,302,310,312,320])]
    #[case(vec![2,2,8,8,2], vec![222,228,282,288,822,828,882])]
    #[case(vec![3,7,5], vec![])]
    fn case(#[case] digits: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::find_even_numbers(digits);
        assert_eq!(actual, expected);
    }
}
