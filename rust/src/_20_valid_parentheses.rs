struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        todo!()
    }
}

#[test]
fn case1() {
    let s = "()";
    let expected = true;

    let actual = Solution::is_valid(s);
    assert_eq!(actual, expected);
}

#[test]
fn case2() {
    let s = "()[]{}";
    let expected = true;

    let actual = Solution::is_valid(s);
    assert_eq!(actual, expected);
}

#[test]
fn case3() {
    let s = "(]";
    let expected = false;

    let actual = Solution::is_valid(s);
    assert_eq!(actual, expected);
}
