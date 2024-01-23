//! Solution for https://leetcode.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters
//! 1239. Maximum Length of a Concatenated String with Unique Characters

impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        // Based on https://leetcode.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters/solutions/2737493/python-c-java-rust-0-ms-bit-set-operations-with-detailed-comments/
        // Couldn't find a non-exponential solution and then found this one that showed that was actually the way to go
        // Edited version of his solution

        // Convert to sets to make intersection easier to check
        let mut max_len: u32 = 0;

        // [1] we should first throw away all strings with any
        //    duplicate characters; strings with all unique
        //    characters are the subsets of the alphabet, thus,
        //    can be encoded using binary form
        let mut unique: Vec<u32> = Vec::new();
        for s in arr {
            // here, we set bits for each of encountered letters
            let bin: u32 = s.bytes().map(|c| 1 << (c - b'a')).sum();
            if bin.count_ones() == s.len() as u32 {
                unique.push(bin);
            }
        }

        // [2] now start with an empty concatenation and iteratively
        //    increase its length by trying to add more strings
        let mut concat: Vec<u32> = vec![0];
        for u in unique {
            for i in 0..concat.len() {
                if (concat[i] & u) == 0 {
                    let cc = concat[i] | u;
                    concat.push(cc);
                    max_len = max_len.max(cc.count_ones());
                }
            }
        }
        max_len as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec!["un".into(),"iq".into(),"ue".into()], 4)]
    #[case(vec!["cha".into(),"r".into(),"act".into(),"ers".into()], 6)]
    #[case(vec!["abcdefghijklmnopqrstuvwxyz".into()], 26)]
    #[case(vec!["abcde".into(),"a".into(),"b".into(),"c".into(),"df".into(),"eh".into()], 7)]
    fn case(#[case] arr: Vec<String>, #[case] expected: i32) {
        let actual = Solution::max_length(arr);
        assert_eq!(actual, expected);
    }
}
