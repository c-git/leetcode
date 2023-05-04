use std::str::Chars;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = String::new();
        let mut iters: Vec<Chars<'_>> = strs.iter().map(|s| s.chars()).collect();
        let mut leader = if let Some(leader) = iters.pop() {
            leader
        } else {
            return result;
        };
        for c in leader {
            for iter in iters.iter_mut() {
                if let Some(x) = iter.next() {
                    if x != c {
                        return result;
                    }
                } else {
                    return result;
                }
            }
            result.push(c);
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
        let input = vec!["flower", "flow", "flight"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "fl";
        let actual = Solution::longest_common_prefix(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input = vec!["dog", "racecar", "car"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "";
        let actual = Solution::longest_common_prefix(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let input = vec!["dog", "dog", "dog"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = "dog";
        let actual = Solution::longest_common_prefix(input);
        assert_eq!(actual, expected);
    }
}
