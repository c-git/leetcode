//! Solution for https://leetcode.com/problems/koko-eating-bananas
//! 875. Koko Eating Bananas

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut low = 1;
        let mut high = *piles.iter().max().unwrap();
        if test_eating(&piles, low, h) {
            return low;
        }
        if !test_eating(&piles, high, h) {
            unreachable!("should never have more piles than hours")
        }
        while low < high {
            let mid = (high + low) / 2;
            if test_eating(&piles, mid, h) {
                high = mid;
            } else if low == mid {
                low += 1;
            } else {
                low = mid;
            }
        }
        high
    }
}

/// Returns true IFF all piles can be eaten in h hours
fn test_eating(piles: &[i32], k: i32, h: i32) -> bool {
    piles
        .iter()
        .map(|x| (x / k) + if x % k == 0 { 0 } else { 1 })
        .sum::<i32>()
        <= h
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3,6,7,11], 8, 4)]
    #[case(vec![30,11,23,4,20], 5, 30)]
    #[case(vec![30,11,23,4,20], 6, 23)]
    fn case(#[case] piles: Vec<i32>, #[case] h: i32, #[case] expected: i32) {
        let actual = Solution::min_eating_speed(piles, h);
        assert_eq!(actual, expected);
    }
}
