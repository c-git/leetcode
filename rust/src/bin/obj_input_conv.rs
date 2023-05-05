use std::{
    env,
    fs::File,
    io::{self, BufRead, Write},
};

use nom::{
    bytes::complete::{is_not, tag},
    character::complete::multispace0,
    combinator::opt,
    IResult,
};

// use clap::Parser;

// #[derive(Parser)]
// #[command(
//     author,
//     version,
//     about,
//     long_about = "This program reads up to the first three lines from the input file
// and outputs the corresponding rust code.
// First line: Specifies the function to call
// Second line: Specifies the arguments
// Third line: (Optional) The value that must be returned from the function call

// Sample cargo command

// cargo run --bin obj_input_conv -- sample_input.txt sample_output.txt

// Example input from https://leetcode.com/problems/lru-cache/ (first function call modified):

// [\"LRUCache::new\", \"put\", \"put\", \"get\", \"put\", \"get\", \"put\", \"get\", \"get\", \"get\"]
// [[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
// [null, null, null, 1, null, -1, null, -1, 3, 4]

// Example Output:
// let mut obj = LRUCache::new(2);
// obj.put(1, 1);
// obj.put(2, 2);
// assert_eq!(obj.get(1), 1);
// obj.put(3, 3);
// assert_eq!(obj.get(2), -1);
// obj.put(4, 4);
// assert_eq!(obj.get(1), -1);
// assert_eq!(obj.get(3), 3);
// assert_eq!(obj.get(4), 4);
// "
// )]
pub struct Cli {
    // #[arg(help = "File to read input from")]
    pub input_file: String,

    // #[arg(help = "File to write output to from")]
    pub output_file: String,
}

fn main() {
    let cli = Cli {
        input_file: env::args().nth(1).expect("No input argument found"),
        output_file: env::args().nth(2).expect("No output argument found"),
    };

    // Get input
    println!("Loading input...");
    let file = File::open(cli.input_file).expect("Failed to read input file");
    let mut file_reader = io::BufReader::new(file).lines();
    let first_line = file_reader
        .next()
        .expect("No first line found")
        .expect("Error reading first line");
    let second_line = file_reader
        .next()
        .expect("No second line found")
        .expect("Error reading second line");
    let third_line = if let Some(line) = file_reader.next() {
        match line {
            Ok(line) => {
                if line.trim() == "" {
                    None
                } else {
                    Some(line)
                }
            }
            Err(e) => {
                eprintln!("Error reading third line: {e}");
                None
            }
        }
    } else {
        None
    };

    // Parse input
    println!("Parsing first line...");
    let first = parse_first_line(first_line.trim());
    println!("Parsing second line...");
    let second = parse_second_line(second_line.trim());
    let third = if let Some(line) = third_line {
        println!("Parsing third line...");
        parse_third_line(line.trim())
    } else {
        println!("No third line");
        vec![None; first.len()]
    };

    // Generate output
    println!("Generating output...");
    debug_assert_eq!(first.len(), second.len());
    debug_assert_eq!(first.len(), third.len());
    let mut output_file = File::create(cli.output_file).expect("Failed to create output file");
    for (i, ((f, s), t)) in first.into_iter().zip(second).zip(third).enumerate() {
        if i == 0 {
            write!(&mut output_file, "let mut obj = ").expect("Failed to write to output");
        }

        let out = format!("obj.{f}({s})");
        if let Some(t) = t {
            writeln!(&mut output_file, "assert_eq!({out}, {t});").ok();
        } else {
            writeln!(&mut output_file, "{out};").expect("Failed to write output");
        }
    }
    println!("Completed");
}

fn parse_first_line_helper(input: &str) -> IResult<&str, Vec<String>> {
    // ["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"]
    let (input, _) = tag("[")(input)?;
    let mut result = vec![];
    let mut remainder = input;
    loop {
        let input = remainder;
        if let Ok((input, _)) = tag::<&str, &str, ()>("]")(input) {
            remainder = input;
            break;
        }
        let (input, _) = tag("\"")(input)?;
        let (input, val) = is_not("\"")(input)?;
        result.push(val.to_string());
        let (input, _) = tag("\"")(input)?;
        let (input, _) = opt(tag(","))(input)?;
        let (input, _) = multispace0(input)?;

        remainder = input;
    }
    Ok((remainder, result))
}
pub fn parse_first_line(msg: &str) -> Vec<String> {
    let (_, result) = parse_first_line_helper(msg).expect("Failed to parse first line");
    result
}

fn parse_second_line_helper(input: &str) -> IResult<&str, Vec<String>> {
    // [[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
    let (input, _) = tag("[")(input)?;
    let mut result = vec![];
    let mut remainder = input;
    loop {
        let input = remainder;
        if let Ok((input, _)) = tag::<&str, &str, ()>("]")(input) {
            remainder = input;
            break;
        }
        let (input, _) = tag("[")(input)?;
        let (input, val) = opt(is_not("]"))(input)?;
        result.push(if let Some(val) = val {
            val.to_string()
        } else {
            "".to_string()
        });
        let (input, _) = tag("]")(input)?;
        let (input, _) = opt(tag(","))(input)?;
        let (input, _) = multispace0(input)?;

        remainder = input;
    }
    Ok((remainder, result))
}
pub fn parse_second_line(msg: &str) -> Vec<String> {
    let (_, result) = parse_second_line_helper(msg).expect("Failed to parse second line");
    result
}

fn parse_third_line_helper(input: &str) -> IResult<&str, Vec<Option<String>>> {
    // [null, null, null, 1, null, -1, null, -1, 3, 4]
    let (input, _) = tag("[")(input)?;
    let mut result = vec![];
    let mut remainder = input;
    loop {
        let input = remainder;
        if let Ok((input, _)) = tag::<&str, &str, ()>("]")(input) {
            remainder = input;
            break;
        }
        let (input, val) = is_not(",]")(input)?;
        result.push(if val == "null" {
            None
        } else {
            Some(val.to_string())
        });
        let (input, _) = opt(tag(","))(input)?;
        let (input, _) = multispace0(input)?;

        remainder = input;
    }
    Ok((remainder, result))
}
pub fn parse_third_line(msg: &str) -> Vec<Option<String>> {
    let (_, result) = parse_third_line_helper(msg).expect("Failed to parse third line");
    result
}
