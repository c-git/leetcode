impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut result = Vec::with_capacity(s.len());
        let mut to_delete: u32 = 0;

        for c in s.chars().rev() {
            match (c, to_delete) {
                (c, _) if c == '*' => to_delete += 1,
                (_, to_del) if to_del > 0 => to_delete -= 1,
                (c, _) => result.push(c),
            }
        }

        result.into_iter().rev().collect()
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = "leet**cod*e".to_owned();
        let expected = "lecoe";
        let actual = Solution::remove_stars(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input = "erase*****".to_owned();
        let expected = "";
        let actual = Solution::remove_stars(input);
        assert_eq!(actual, expected);
    }
}
