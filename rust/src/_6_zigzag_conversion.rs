impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 1 {
            return s;
        }

        let num_rows = num_rows as usize;
        let mut lines = vec!["".to_string(); num_rows];

        let mut row = 0;
        let mut col = 0;
        for c in s.chars() {
            lines[row].push(c);
            // println!("c = {c}, {row},{col} (row,col)");
            match (row, col) {
                (r, 0) if r < num_rows - 1 => row += 1,
                (_, _) => {
                    row -= 1;
                    col = (col + 1) % (num_rows - 1);
                }
            }
        }

        let mut result = String::with_capacity(s.len());
        for line in lines {
            result.push_str(&line)
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
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 3;
        let expected = "PAHNAPLSIIGYIR";
        let actual = Solution::convert(s, num_rows);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 4;
        let expected = "PINALSIGYAHRPI";
        let actual = Solution::convert(s, num_rows);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let s = "A".to_string();
        let num_rows = 1;
        let expected = "A";
        let actual = Solution::convert(s, num_rows);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case4() {
        let s = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
        let num_rows = 6;
        let expected = "AKUBJLTVCIMSWDHNRXEGOQYFPZ";
        let actual = Solution::convert(s, num_rows);
        assert_eq!(actual, expected);
    }
}
