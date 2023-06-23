//! Solution for https://leetcode.com/problems/can-place-flowers
//! 605. Can Place Flowers

impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        if n == 0 {
            return true;
        }

        debug_assert_ne!(flowerbed.len(), 0);

        let len = flowerbed.len();
        if len == 1 {
            if n <= 1 && flowerbed[0] == 0 {
                return true;
            }
            return false;
        }

        let mut available_spots = 0;

        // Check if ends are available
        if flowerbed[0] == 0 && flowerbed[1] == 0 {
            flowerbed[0] = 1;
            available_spots += 1;
        }

        if flowerbed[len - 1] == 0 && flowerbed[len - 2] == 0 {
            // len must be greater than 2 for this to matter because if it is 2 then
            // it can either be the other spot or this one and if this one if full
            // it's not this one, and if we counted the other one then it's still not this one
            available_spots += 1;
            flowerbed[len - 1] = 1;
        }

        let mut i = 1;
        while i < len - 1 {
            if flowerbed[i - 1] == 0 && flowerbed[i] == 0 && flowerbed[i + 1] == 0 {
                // Values not updated in this loop as I couldn't think of any more missing edge cases
                // and the cost of making the changes didn't seem worthwhile

                available_spots += 1;
                // flowerbed[i] = 1;
                i += 2; // Skip the next spot it can no longer be used
            } else {
                i += 1; // Check if next spot works
            }
        }
        available_spots >= n
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
    #[case(vec![1,0,0,0,0,1], 2, false)]
    #[case(vec![0], 1, true)]
    #[case(vec![0,1,0], 0, true)]
    #[case(vec![0,0], 2, false)]
    #[case(vec![0,0,0,0], 3, false)]
    fn case(#[case] flowerbed: Vec<i32>, #[case] n: i32, #[case] expected: bool) {
        let actual = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(actual, expected);
    }
}
