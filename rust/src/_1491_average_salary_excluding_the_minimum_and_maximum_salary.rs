impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        debug_assert!(salary.len() >= 3);
        // Ok to use i32 to sum because max total salaries is 10^8
        let mut sum = 0;
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for &sal in &salary {
            sum += sal;
            if min > sal {
                min = sal;
            }
            if max < sal {
                max = sal;
            }
        }
        sum -= min + max;
        sum as f64 / (salary.len() - 2) as f64
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = vec![4000, 3000, 1000, 2000];
        let expected = 2500.0;
        let actual = Solution::average(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input = vec![1000, 2000, 3000];
        let expected = 2000.0;
        let actual = Solution::average(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let input = vec![1_000_000; 100];
        let expected = 1_000_000.0;
        let actual = Solution::average(input);
        assert_eq!(actual, expected);
    }
}
