impl Solution {
    pub fn remove_duplicates(nums: &mut [i32]) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut last_pos = 0;
        for i in 1..nums.len() {
            if nums[last_pos] != nums[i] {
                last_pos += 1;
                nums[last_pos] = nums[i];
            }
        }
        last_pos as i32 + 1
    }
}

pub struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut input = vec![1, 1, 2];
        let expected = [1, 2];
        let result_len = Solution::remove_duplicates(&mut input);
        assert_eq!(&input[..result_len as usize], &expected);
    }

    #[test]
    fn case2() {
        let mut input = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let expected = [0, 1, 2, 3, 4];
        let result_len = Solution::remove_duplicates(&mut input);
        assert_eq!(&input[..result_len as usize], &expected);
    }

    #[test]
    fn case3() {
        let mut input = vec![3];
        let expected = [3];
        let result_len = Solution::remove_duplicates(&mut input);
        assert_eq!(&input[..result_len as usize], &expected);
    }

    #[test]
    fn case4() {
        let mut input = vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5];
        let expected = [5];
        let result_len = Solution::remove_duplicates(&mut input);
        assert_eq!(&input[..result_len as usize], &expected);
    }
}
