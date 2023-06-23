//! Solution for https://leetcode.com/problems/can-place-flowers
//! 605. Can Place Flowers

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        if n == 0 {
            return true;
        }

        // Check constraints used
        debug_assert!(!flowerbed.is_empty());
        debug_assert!(flowerbed.iter().all(|x| (0..=1).contains(x)));

        let max_flowers = (flowerbed.len() + 1) / 2; //Plus 1 to round up
        let planted_flowers = flowerbed.iter().sum::<i32>() as usize;
        debug_assert!(max_flowers >= planted_flowers);
        max_flowers - planted_flowers >= n as usize
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,0,0,0,1], 1, true)]
    #[case(vec![1,0,0,0,1], 2,false)]
    #[case(vec![0,1,0], 1,false)]
    fn case(#[case] flowerbed: Vec<i32>, #[case] n: i32, #[case] expected: bool) {
        let actual = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(actual, expected);
    }
}
