impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let k = k as usize;
        let s: Vec<char> = s.chars().collect();
        let vowels = "aeiou";

        let mut result: usize = s
            .iter()
            .take(k)
            .filter_map(|&c| if vowels.contains(c) { Some(1) } else { None })
            .sum();

        let mut curr_window_count = result;
        for i in (0..s.len()).skip(k) {
            if vowels.contains(s[i]) {
                curr_window_count += 1;
            }
            if vowels.contains(s[i - k]) {
                curr_window_count -= 1;
            }
            if result < curr_window_count {
                result = curr_window_count;
                if result == k {
                    break;
                }
            }
        }

        result as i32
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "abciiidef".into();
        let k = 3;
        let expected = 3;
        let actual = Solution::max_vowels(s, k);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let s = "aeiou".into();
        let k = 2;
        let expected = 2;
        let actual = Solution::max_vowels(s, k);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let s = "leetcode".into();
        let k = 3;
        let expected = 2;
        let actual = Solution::max_vowels(s, k);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case4() {
        let s = "nvt".into();
        let k = 5;
        let expected = 0;
        let actual = Solution::max_vowels(s, k);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case5() {
        let s = "nvt".into();
        let k = 1;
        let expected = 0;
        let actual = Solution::max_vowels(s, k);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case6() {
        let s = "aanvtmynqwrt".into();
        let k = 5;
        let expected = 2;
        let actual = Solution::max_vowels(s, k);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case7() {
        let s = "nvtmynqwrtaa".into();
        let k = 5;
        let expected = 2;
        let actual = Solution::max_vowels(s, k);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case8() {
        let s = "nvtaaamynqwrtaaab".into();
        let k = 3;
        let expected = 3;
        let actual = Solution::max_vowels(s, k);
        assert_eq!(actual, expected);
    }
}
