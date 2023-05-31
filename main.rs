use regex::Regex;
use std::{env, fs::File, io::Write};

fn main() {
    //Get args from command line
    let args: Vec<String> = env::args().collect();
    // if only thename of program 
    if args.len() == 1 {
        help();
    }// if less arguments then 2 
    else if args.len() < 3 {
        println!("Expected 2 arguments");
        help();
    } else {
        let file_name = &args[1];
        let file_path = &args[2];
        let file_path_string = file_path.to_string();

        let code_blocks = extract_code_blocks(&file_path_string);
        create_file_and_write(&code_blocks, file_name).expect("Something went wrong");
    }
}

// Extracts code blocks from the file into a Vec of strings
fn extract_code_blocks(file_path: &str) -> Vec<String> {
    // Open a file and convert it to a string
    let contents = std::fs::read_to_string(file_path).expect("Wrong Path");

    // Regular expression to find all encapsulated contents inside ` `
    let pattern = Regex::new(r"```([\s\S]*?)```").unwrap();

    // Regular expression to find the first word of the string
    let language_pattern = Regex::new(r"^\w+").unwrap();

    // Match if the file contains code blocks
    // If it doesn't, close the program
    match pattern.captures(&contents) {
        Some(captures) => {
            let code_blocks: Vec<String> = captures
                .iter()
                // First get rid of `, then drop the first word
                // (it contains the name of the programming language)
                .map(|capture| {
                    language_pattern
                        .replace_all(&capture.unwrap().as_str().replace(r"`", ""), "")
                        .to_string()
                })
                .collect();
            return code_blocks;
        }
        None => {
            panic!("No code blocks found in the file");
        }
    }
}
//Apparently rust allows me to write a file in bites wich it is weird i have to do more reascerch
//obn ths !! 
fn create_file_and_write(code_blocks: &Vec<String>, path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    let contents = code_blocks.join("\n");
    file.write_all(contents.as_bytes())?;
    Ok(())
}
// Just the usage call
fn help() {
    println!(
        "Usage:
  formatter   file name   [path]
This program takes a file name and path to a markdown file.
Then it  produces a file with extracted code blocks from the given markdown.
Example:
formatter test.py test.md
"
    );

    std::process::exit(1);}
