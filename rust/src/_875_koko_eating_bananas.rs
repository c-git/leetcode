//! Solution for https://leetcode.com/problems/koko-eating-bananas
//! 875. Koko Eating Bananas

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut low = 1;
        let mut high = *piles.iter().max().unwrap();
        debug_assert!(test_can_eat_all(&piles, h, high));
        while low < high {
            let mid = (low + high) / 2;
            if test_can_eat_all(&piles, h, mid) {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        high
    }
}

fn test_can_eat_all(piles: &[i32], h: i32, rate: i32) -> bool {
    let mut hours_taken = 0;
    for pile in piles {
        hours_taken += pile / rate;
        if pile % rate > 0 {
            hours_taken += 1;
        }
        if hours_taken > h {
            return false;
        }
    }
    hours_taken <= h
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
