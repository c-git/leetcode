impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        // Observation: Based on the constraints there are exactly n-1 edges therefore each edge must go to the center
        // So we only need to find the node in common to the first two edges because they must both go to the center
        if edges[1].contains(&edges[0][0]) {
            edges[0][0]
        } else {
            debug_assert!(edges[1].contains(&edges[0][1]));
            edges[0][1]
        }
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = vec![vec![1, 2], vec![2, 3], vec![4, 2]];
        let expected = 2;
        let actual = Solution::find_center(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input = vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]];
        let expected = 1;
        let actual = Solution::find_center(input);
        assert_eq!(actual, expected);
    }
}
