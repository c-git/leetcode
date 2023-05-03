impl Solution {
    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort_unstable();
        nums2.sort_unstable();
        let mut nums1 = nums1.into_iter();
        let mut nums2 = nums2.into_iter();
        let mut result = vec![];
        let mut curr1 = nums1.next();
        let mut curr2 = nums2.next();

        #[allow(clippy::while_let_loop)]
        loop {
            let num1 = if let Some(num) = curr1 { num } else { break };
            let num2 = if let Some(num) = curr2 { num } else { break };
            match num1.cmp(&num2) {
                std::cmp::Ordering::Less => {
                    //num1 is too small will never match
                    curr1 = nums1.next();
                    curr2 = Some(num2);
                }
                std::cmp::Ordering::Equal => {
                    result.push(num1);
                    curr1 = nums1.next();
                    curr2 = nums2.next();
                }
                std::cmp::Ordering::Greater => {
                    //num2 is too small will never match
                    curr1 = Some(num1);
                    curr2 = nums2.next();
                }
            }
        }
        result
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let expected = vec![2, 2];
        let actual = Solution::intersect(nums1, nums2);
        validator(actual, expected);
    }

    #[test]
    fn case2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        let expected = vec![9, 4];
        let actual = Solution::intersect(nums1, nums2);
        validator(actual, expected);
    }

    fn validator(mut actual: Vec<i32>, mut expected: Vec<i32>) {
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }
}
