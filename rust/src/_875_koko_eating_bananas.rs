//! Solution for https://leetcode.com/problems/koko-eating-bananas
//! 875. Koko Eating Bananas

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut size = *piles.iter().max().unwrap();
        debug_assert!(test_can_eat_all(size, &piles, h), "{size}");
        let mut base = 1;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            base = if !test_can_eat_all(half + base, &piles, h) {
                mid
            } else {
                base
            };
            size -= half;
        }
        if !test_can_eat_all(base, &piles, h) {
            base + 1
        } else {
            base
        }
    }
}

fn test_can_eat_all(test_value: i32, piles: &[i32], h: i32) -> bool {
    let mut hours_sum = 0;
    for pile in piles {
        hours_sum += pile / test_value;
        if pile % test_value > 0 {
            hours_sum += 1;
        }
        if hours_sum > h {
            return false;
        }
    }
    true
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
    #[case(vec![332484035,524908576,855865114,632922376,222257295,690155293,112677673,679580077,337406589,290818316,877337160,901728858,679284947,688210097,692137887,718203285,629455728,941802184], 823855818, 14)]
    fn case(#[case] piles: Vec<i32>, #[case] h: i32, #[case] expected: i32) {
        let actual = Solution::min_eating_speed(piles, h);
        assert_eq!(actual, expected);
    }
}
