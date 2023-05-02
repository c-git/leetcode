impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        println!("{s}");
        let s = s.chars().collect::<Vec<char>>();
        let mut longest_end_index = 0;
        let mut longest_len = 1;

        // Stores the longest palindrome in the subarray up to that point in the table
        // Starts at 1 because each char is a minimal palindrome
        let mut table = vec![1; s.len()];

        for i in 1..s.len() {
            // println!("B i: {i} Table: {table:?}");
            let adjacent = if s[i] == s[i - 1] { 2 } else { 1 }; // Check if can pair with adjacent cell
            let extend = if table[i - 1] < i && s[i] == s[i - table[i - 1] - 1] {
                table[i - 1] + 2
            } else {
                1
            };
            table[i] = adjacent.max(extend);
            if table[i] > longest_len {
                longest_len = table[i];
                longest_end_index = i;
            }
            println!("A i: {i} Table: {table:?} adjacent: {adjacent} extend: {extend}, longest_index: {longest_end_index}");
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
}
