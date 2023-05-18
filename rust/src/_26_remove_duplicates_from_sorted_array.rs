impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut sorted_len = 1;
        let n = nums.len();
        let mut i = 1;
        while i < n {
            if nums[i] != nums[sorted_len - 1] {
                nums.swap(i, sorted_len);
                sorted_len += 1;
            }
            i += 1;
        }
        sorted_len as i32
    }
}

struct Solution;
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
