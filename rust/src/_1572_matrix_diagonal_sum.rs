impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let mut result = if n % 2 == 1 { -mat[n / 2][n / 2] } else { 0 };
        for (i, row) in mat.iter().enumerate() {
            result += row[i] + row[n - i - 1];
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
        let input = [
            [1, 2, 3], // Comment put put on multiple
            [4, 5, 6],
            [7, 8, 9],
        ];
        let input = input.into_iter().map(|x| x.to_vec()).collect();
        let expected = 25;
        let actual = Solution::diagonal_sum(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input = [
            [1, 1, 1, 1], // Comment put put on multiple
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 1, 1],
        ];
        let input = input.into_iter().map(|x| x.to_vec()).collect();
        let expected = 8;
        let actual = Solution::diagonal_sum(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let input = [[5]];
        let input = input.into_iter().map(|x| x.to_vec()).collect();
        let expected = 5;
        let actual = Solution::diagonal_sum(input);
        assert_eq!(actual, expected);
    }
}
