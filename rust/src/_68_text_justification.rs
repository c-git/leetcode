//! Solution for https://leetcode.com/problems/text-justification
//! 68. Text Justification

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut result = vec![];
        let mut line_start = 0;
        let mut words_width = 0;
        for (line_end, word) in words.iter().enumerate() {
            // LI:
            //    - All words before line_start are already dealt with (ie. Included in result)
            //    - All the words widths from line_start to before line_end add up to words_width
            //    - words_width + a single space between words is less than or equal to max_width
            if words_width + word.len() + line_end - line_start <= max_width {
                words_width += word.len();
            } else {
                debug_assert!(line_end > line_start, "Constraints say that all words are at most max length and hence would have gone to the other branch");
                let num_gaps = line_end - line_start - 1;
                let smaller_space_width = if num_gaps == 0 {
                    0
                } else {
                    (max_width - words_width) / (num_gaps)
                };
                let extra_spaces_needed = max_width - words_width - smaller_space_width * num_gaps;
                let mut line = String::with_capacity(max_width);
                let smaller_space = " ".repeat(smaller_space_width);
                for (gap_num, word_for_line) in words
                    .iter()
                    .skip(line_start)
                    .take(line_end - line_start)
                    .enumerate()
                {
                    line.push_str(word_for_line);
                    if gap_num < num_gaps {
                        line.push_str(&smaller_space);
                        if gap_num < extra_spaces_needed {
                            line.push(' ');
                        }
                    }
                }
                line.push_str(&" ".repeat(max_width - line.len())); // Should only be used on single words
                debug_assert_eq!(line.len(), max_width, "line: {line:?}");
                line_start = line_end;
                words_width = word.len();
                result.push(line);
            }
        }

        // Do last line
        debug_assert!(line_start < words.len());
        let mut last_line = String::with_capacity(max_width);
        for (i, word) in words.iter().enumerate().skip(line_start) {
            last_line.push_str(word);
            if i < words.len() - 1 {
                last_line.push(' ');
            }
        }
        last_line.push_str(&" ".repeat(max_width - last_line.len()));
        result.push(last_line);
        result
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec!["This".into(), "is".into(), "an".into(), "example".into(), "of".into(), "text".into(), "justification.".into()], 16, 
        vec![
            "This    is    an".into(),
            "example  of text".into(),
            "justification.  ".into()
        ]
    )]
    #[case(vec!["What".into(),"must".into(),"be".into(),"acknowledgment".into(),"shall".into(),"be".into()], 16, 
        vec![
            "What   must   be".into(),
            "acknowledgment  ".into(),
            "shall be        ".into()
        ]
     )]
    #[case(vec!["Science".into(),"is".into(),"what".into(),"we".into(),"understand".into(),"well".into(),"enough".into(),"to".into(),"explain".into(),"to".into(),"a".into(),"computer.".into(),"Art".into(),"is".into(),"everything".into(),"else".into(),"we".into(),"do".into()], 20, 
        vec![
            "Science  is  what we".into(),
            "understand      well".into(),
            "enough to explain to".into(),
            "a  computer.  Art is".into(),
            "everything  else  we".into(),
            "do                  ".into()
        ]
    )]
    fn case(#[case] words: Vec<String>, #[case] max_width: i32, #[case] expected: Vec<String>) {
        let actual = Solution::full_justify(words, max_width);
        assert_eq!(actual, expected);
    }
}
