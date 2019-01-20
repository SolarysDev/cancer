extern crate colored;
extern crate clipboard;

use std::env;
use colored::*;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut output = String::new();
    match args[1].as_str() {
        "-h" => {
            println!("{} -> bold itaics, with spaces between the characters", "spaced".green().bold());
            println!("{} -> put a :clap: between each word", "clap".yellow().bold());
            println!("{} -> clap but every alphabet character is emojified", "emojiclap".cyan().bold());
        }
        "spaced" => {
            args.drain(0..2);
            let args = args.join(" ");
            output = space(args);
        }
        "clap" => {
            args.drain(0..2);
            let args = args.join(" ");
            output = clap(args);
        }
        "emojiclap" => {
            args.drain(0..2);
            let args = args.join(" ");
            output = emoji_clap(args);
        }
        _ => {
            eprintln!("err: {} is not a valid mutator. use -h.", args[1]);
        }
    }

    println!("{}", output);
}

fn space(input: String) -> String {
    let mut output = String::from("***");
    let input: Vec<char> = input.chars().collect();

    for i in 0..input.len() {
        output.push(input[i]);
        if i != input.len() - 1 {
            output.push(' ');
        }
    }
    output.push_str("***");
    return output;
}

fn clap(input: String) -> String {
    let output = input.replace(" ", " :clap: ");
    return output;
}

fn emoji_clap(input: String) -> String {
    let mut output = String::new();
    let input: Vec<char> = input.chars().collect();

    for i in 0..input.len() {
        if input[i].is_alphabetic() {
            output.push_str(format!(":regional_indicator_{}: ", input[i].to_lowercase()).as_str());
        }
            else if input[i] == ' ' {
                output.push_str(" :clap: ");
            }
            else {
                output.push(input[i]);
            }


    }
    return output;
}
