struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_diff = 0;
        let mut min_price = i32::MAX;
        for price in prices {
            if price < min_price {
                min_price = price;
            } else {
                let diff = price - min_price;
                if diff > max_diff {
                    max_diff = diff;
                }
            }
        }
        max_diff
    }
}

#[test]
fn case1() {
    let input = vec![7, 1, 5, 3, 6, 4];
    let expected = 5;
    let actual = Solution::max_profit(input);
    assert_eq!(actual, expected);
}

#[test]
fn case2() {
    let input = vec![7, 6, 4, 3, 1];
    let expected = 0;
    let actual = Solution::max_profit(input);
    assert_eq!(actual, expected);
}
