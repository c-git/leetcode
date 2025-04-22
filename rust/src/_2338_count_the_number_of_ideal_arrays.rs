//! Solution for https://leetcode.com/problems/count-the-number-of-ideal-arrays
//! 2338. Count the Number of Ideal Arrays

impl Solution {
    /// Converted version of https://leetcode.com/problems/count-the-number-of-ideal-arrays/solutions/6675631/beat-100-with-explanation/
    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        const MODULO: i32 = 1_000_000_007;
        const MAX_VALUE: i32 = 10_000;

        let mut strict_counts: Vec<Vec<i32>> = vec![(1..=MAX_VALUE).collect()];
        let mut prev_row = vec![1; MAX_VALUE as usize];
        let mut next_row = vec![0; MAX_VALUE as usize];
        let mut prev_base = 1;

        while (prev_base << 1) <= MAX_VALUE {
            let next_base = prev_base << 1;
            next_row
                .iter_mut()
                .skip(next_base as usize - 1)
                .for_each(|x| *x = 0);
            for prev_num in prev_base..=MAX_VALUE {
                let prev_count = prev_row[(prev_num - 1) as usize];
                for multiple in 2..=MAX_VALUE {
                    let product = prev_num * multiple;
                    if product > MAX_VALUE {
                        break;
                    }
                    next_row[product as usize - 1] =
                        (next_row[product as usize - 1] + prev_count) % MODULO;
                }
            }
            let mut current_counts = vec![next_row[next_base as usize - 1]];
            for next_num in next_base..MAX_VALUE {
                current_counts
                    .push((*current_counts.last().unwrap() + next_row[next_num as usize]) % MODULO)
            }
            strict_counts.push(current_counts);
            prev_base = next_base;
            (prev_row, next_row) = (next_row, prev_row);
        }

        let mut count = 0;
        let mut combo = 1;
        let mut top = n - 1;
        let mut bottom = 1;
        let mut base = 1;
        #[expect(clippy::needless_range_loop)]
        for k in 0..strict_counts.len().min(n as _) {
            if base <= max_value {
                count = (count + combo * strict_counts[k][(max_value - base) as usize]) % MODULO;
            } else {
                break;
            }
            combo = combo * top / bottom;
            top -= 1;
            bottom += 1;
            base <<= 1;
        }
        count
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(2, 5, 10)]
    #[case(5, 3, 11)]
    fn case(#[case] n: i32, #[case] max_value: i32, #[case] expected: i32) {
        let actual = Solution::ideal_arrays(n, max_value);
        assert_eq!(actual, expected);
    }
}
