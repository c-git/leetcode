//! Solution for https://leetcode.com/problems/simplify-path
//! 71. Simplify Path

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut folders = vec![];
        let mut start_index = 0;
        debug_assert_eq!(&path[0..1], "/");
        let mut is_prev_slash = true;
        for (i, c) in path.char_indices().skip(1) {
            let is_curr_slash = c == '/';
            match (is_prev_slash, is_curr_slash) {
                (true, true) => {
                    // One or more slashes is still just a slash, but move start index forward to track we are not currently reading a folder
                    start_index = i;
                }
                (true, false) => {
                    // Start of a new folder
                    start_index = i;
                }
                (false, true) => {
                    // End of a folder
                    Self::process_folder(&path[start_index..i], &mut folders);
                    start_index = i;
                }
                (false, false) => (), // Do nothing still continuing with folder name
            }
            is_prev_slash = is_curr_slash;
        }
        if &path[start_index..start_index + 1] != "/" {
            // Handle last folder
            Self::process_folder(&path[start_index..], &mut folders);
        }

        if folders.is_empty() {
            return "/".to_string();
        }
        let mut result = String::new();
        for folder in folders {
            result.push('/');
            result.push_str(folder);
        }
        result
    }
    fn process_folder<'a>(folder: &'a str, folders: &mut Vec<&'a str>) {
        match folder {
            "." => (), // Do nothing no new folder to add
            ".." => {
                // Remove previous folder because we went back up
                folders.pop();
            }
            _ => {
                // Save current folder
                folders.push(folder);
            }
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("/home/", "/home")]
    #[case("/home", "/home")]
    #[case("/", "/")]
    #[case("/////.", "/")]
    #[case("/////..", "/")]
    #[case("/////...", "/...")]
    #[case("/home//foo/", "/home/foo")]
    #[case("/home/user/Documents/../Pictures", "/home/user/Pictures")]
    #[case("/.../a/../b/c/../d/./", "/.../b/d")]
    #[case("/../", "/")]
    fn case(#[case] path: String, #[case] expected: String) {
        let actual = Solution::simplify_path(path);
        assert_eq!(actual, expected);
    }
}
