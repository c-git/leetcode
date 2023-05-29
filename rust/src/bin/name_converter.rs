use std::env;

use regex::Regex;

fn main() {
    let problem_name = env::args().skip(1).collect::<Vec<_>>().join(" ");
    let mut converted_name = convert_name(problem_name);
    converted_name.push_str(".rs"); // Add rust extension
    println!("\nNew Name is: \n{converted_name}\n");
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
