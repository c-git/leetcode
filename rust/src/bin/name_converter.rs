use std::env;

use regex::Regex;

fn main() {
    println!("\n{}\n", convert_name());
}

fn convert_name() -> String {
    let mut result = env::args().skip(1).collect::<Vec<_>>().join(" ");

    // Check for no input
    if result.is_empty() {
        return "
Expected the name to be converted to be passed as argument(s) to the function
Eg. \"cargo run -- 2215. Find the Difference of Two Arrays\"
"
        .to_string();
    }

    // Remove invalid characters
    result = result.replace('.', "");
    let re = Regex::new("[^_A-Za-z0-9]").unwrap();
    result = re.replace_all(&result, "_").to_lowercase();

    //Trim any _ at start or end (could be from brackets or other char not just spaces so done now not earlier)
    result = result.trim_matches('_').to_string();

    // Add rust extension
    result.push_str(".rs");

    // Handle if leading character is a number
    if result.starts_with(char::is_numeric) {
        let mut temp = "_".to_string();
        temp.push_str(&result);
        result = temp;
    }

    result
}
