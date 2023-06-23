//! Solution for https://leetcode.com/problems/reverse-vowels-of-a-string
//! 345. Reverse Vowels of a String

impl Solution {
    pub fn reverse_vowels(mut s: String) -> String {
        let vowels = "aeiouAEIOU";
        let vowels_indices: Vec<usize> = s
            .chars()
            .enumerate()
            .filter_map(|(i, c)| if vowels.contains(c) { Some(i) } else { None })
            .collect();
        let s_vec = unsafe { s.as_mut_vec() };
        let mut lower = 0;
        let mut upper = vowels_indices.len() - 1;
        while lower < upper {
            s_vec.swap(vowels_indices[lower], vowels_indices[upper]);
            lower += 1;
            upper -= 1;
        }
        s
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("hello", "holle")]
    #[case("leetcode", "leotcede")]
    fn case(#[case] s: String, #[case] expected: String) {
        let actual = Solution::reverse_vowels(s);
        assert_eq!(actual, expected);
    }
}
