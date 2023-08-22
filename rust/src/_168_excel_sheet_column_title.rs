//! Solution for https://leetcode.com/problems/excel-sheet-column-title
//! 168. Excel Sheet Column Title

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut result = vec![];
        let mut column_number = column_number as usize;
        let base = 26;
        let mut chars = Vec::with_capacity(25);
        chars.push('Z');
        #[allow(clippy::almost_complete_range)]
        chars.extend('A'..'Z');
        while column_number > 0 {
            let last = column_number % base;
            result.push(chars[last]);
            column_number /= base;
            if last == 0 {
                column_number -= 1;
            }
        }
        result.into_iter().rev().collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(1, "A".into())]
    #[case(28, "AB".into())]
    #[case(701, "ZY".into())]
    #[case(2, "B".into())]
    #[case(26, "Z".into())]
    #[case(27, "AA".into())]
    #[case(52, "AZ".into())]
    fn case(#[case] column_number: i32, #[case] expected: String) {
        let actual = Solution::convert_to_title(column_number);
        assert_eq!(actual, expected);
    }
}
