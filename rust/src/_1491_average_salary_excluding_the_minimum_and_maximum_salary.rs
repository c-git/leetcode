impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        debug_assert!(salary.len() >= 3);
        let mut sum = 0.0;
        let mut min = f64::MAX;
        let mut max = f64::MIN;
        for sal in &salary {
            let sal = *sal as f64;
            sum += sal;
            if min > sal {
                min = sal;
            }
            if max < sal {
                max = sal;
            }
        }
        sum -= min + max;
        sum / (salary.len() - 2) as f64
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
}
