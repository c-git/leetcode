use std::{env, fs::OpenOptions, io::Write};

use regex::Regex;

fn main() {
    // Get input and convert to output filename
    let problem_name = env::args().skip(1).collect::<Vec<_>>().join(" ");
    let converted_name = convert_name(problem_name);
    println!("\nNew Name is: \n{converted_name}\n");

    // Create new file
    let filename = format!("src/{converted_name}.rs");
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)
        .expect("Failed to open file for writing");
    file.write_all(&get_boilerplate())
        .expect("Failed to write to file");
    println!("Created New file");

    // Update lib.rs
    let filename = "src/lib.rs";
    let mut file = OpenOptions::new()
        .append(true)
        .open(filename)
        .expect("Failed to open lib.rs");
    file.write_all(format!("mod {converted_name};").as_bytes())
        .expect("Failed to update lib.rs");
    println!("Added to lib.rs");
}

fn get_boilerplate() -> Vec<u8> {
    "

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(,)]
    fn case(#[case] input: , #[case] expected: ) {
        let actual = Solution::(input);
        assert_eq!(actual, expected);
    }
}"
    .into()
}

fn convert_name(problem_name: String) -> String {
    let mut result = problem_name;

    // Check for no input
    if result.is_empty() {
        panic!(
            "
Expected the name to be converted to be passed as argument(s) to the function
Eg. \"cargo run -- 2215. Find the Difference of Two Arrays\"
"
        )
    }

    // Remove invalid characters
    result = result.replace('.', "");
    let re = Regex::new("[^_A-Za-z0-9]").unwrap();
    result = re.replace_all(&result, "_").to_lowercase();

    //Trim any _ at start or end (could be from brackets or other char not just spaces so done now not earlier)
    result = result.trim_matches('_').to_string();

    // Handle if leading character is a number
    if result.starts_with(char::is_numeric) {
        let mut temp = "_".to_string();
        temp.push_str(&result);
        result = temp;
    }

    result
}
