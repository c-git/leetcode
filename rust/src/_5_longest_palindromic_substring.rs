impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        // println!("{s}");
        let s = s.chars().collect::<Vec<char>>();
        let mut longest_end_index = 0;
        let mut longest_len = 1;

        // Stores the longest palindrome in the subarray up to that point in the table
        // Starts at 1 because each char is a minimal palindrome
        let mut table = vec![1; s.len()];

        if s.len() >= 2 && s[0] == s[1] {
            // Used to extract check from each iteration of the loop
            table[1] = 2;
            longest_end_index = 1;
            longest_len = 2;
        }

        for i in 2..s.len() {
            let adjacent = if s[i] == s[i - 1] {
                // Two in a row
                2
            } else {
                1 // No match but still equal to itself
            };

            // Check if an existing palindrome can be extended
            let extend_by_2 = if table[i - 1] < i && s[i] == s[i - table[i - 1] - 1] {
                table[i - 1] + 2
            } else {
                1
            };

            // Check if can be extended by 1
            let extend_by_1 = if table[i - 1] - 1 == table[i - 2] && s[i] == s[i - table[i - 1]] {
                table[i - 1] + 1
            } else {
                1
            };

            table[i] = adjacent.max(extend_by_2).max(extend_by_1);
            if table[i] > longest_len {
                longest_len = table[i];
                longest_end_index = i;
            }
            //println!("A i: {i} Table: {table:?} adjacent: {adjacent}, extend_by_1: {extend_by_1}, extend_by_2: {extend_by_2}, longest_index: {longest_end_index}");
        }

        s.iter()
            .skip(longest_end_index + 1 - table[longest_end_index])
            .take(table[longest_end_index])
            .collect()
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    fn evaluator(input: &str, actual: &str, expected: &str) {
        if actual == expected
            || (actual.len() == expected.len() && input.contains(actual) && is_palindrome(actual))
        {
            // Do nothing test passes
        } else {
            // To trigger failed test
            assert_eq!(actual, expected);
        }
    }

    fn is_palindrome(actual: &str) -> bool {
        let actual: Vec<char> = actual.chars().collect();
        let mut left = 0;
        let mut right = actual.len() - 1;
        while left < right {
            if actual[left] == actual[right] {
                left += 1;
                right -= 1;
            } else {
                return false;
            }
        }
        true
    }

    #[test]
    fn case1() {
        let input = "babad".to_string();
        let expected = "bab";
        let actual = Solution::longest_palindrome(input.clone());
        evaluator(&input, &actual, expected);
    }

    #[test]
    fn case2() {
        let input = "cbbd".to_string();
        let expected = "bb";
        let actual = Solution::longest_palindrome(input.clone());
        evaluator(&input, &actual, expected);
    }

    #[test]
    fn case3() {
        let input = "ccc".to_string();
        let expected = "ccc";
        let actual = Solution::longest_palindrome(input.clone());
        evaluator(&input, &actual, expected);
    }

    #[test]
    fn case4() {
        let input = "adcbaabcdf".to_string();
        let expected = "dcbaabcd";
        let actual = Solution::longest_palindrome(input.clone());
        evaluator(&input, &actual, expected);
    }

    #[test]
    fn case5() {
        let input = "adcbabcdf".to_string();
        let expected = "dcbabcd";
        let actual = Solution::longest_palindrome(input.clone());
        evaluator(&input, &actual, expected);
    }

    #[test]
    fn case6() {
        let input = "dcbabcd".to_string();
        let expected = "dcbabcd";
        let actual = Solution::longest_palindrome(input.clone());
        evaluator(&input, &actual, expected);
    }

    #[test]
    fn case7() {
        let input = "aaaaaaa".to_string();
        let expected = "aaaaaaa";
        let actual = Solution::longest_palindrome(input.clone());
        evaluator(&input, &actual, expected);
    }
}
