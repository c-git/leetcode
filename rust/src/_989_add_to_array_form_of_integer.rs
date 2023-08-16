//! Solution for https://leetcode.com/problems/add-to-array-form-of-integer
//! 989. Add to Array-Form of Integer

impl Solution {
    pub fn add_to_array_form(mut num: Vec<i32>, mut k: i32) -> Vec<i32> {
        // Stores result in input array

        let mut carry = 0;
        for digit in num.iter_mut().rev() {
            let val = *digit + k % 10 + carry; // Calculate sum for this position
            *digit = val % 10; // Set this position to the last digit of the sum
            carry = val / 10; // Store val without last digit in carry
            k /= 10; // Drop last digit
        }
        if carry > 0 {
            num.insert(0, carry);
        }
        num
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,0,0], 34, vec![1,2,3,4])]
    #[case(vec![2,7,4], 181, vec![4,5,5])]
    #[case(vec![2,1,5], 806, vec![1,0,2,1])]
    fn case(#[case] num: Vec<i32>, #[case] k: i32, #[case] expected: Vec<i32>) {
        let actual = Solution::add_to_array_form(num, k);
        assert_eq!(actual, expected);
    }
}
