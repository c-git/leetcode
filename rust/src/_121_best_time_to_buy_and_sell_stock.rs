struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        todo!()
    }
}

#[test]
fn case1() {
    let input = vec![7, 1, 5, 3, 6, 4];
    let expected = 5;
    let actual = Solution::max_profit(input);
    assert_eq!(expected, actual);
}

#[test]
fn case2() {
    let input = vec![7, 6, 4, 3, 1];
    let expected = 0;
    let actual = Solution::max_profit(input);
    assert_eq!(expected, actual);
}
