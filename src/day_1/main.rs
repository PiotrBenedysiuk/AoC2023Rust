use std::{env, io};
use std::fs::File;
use std::io::BufRead;
const INPUT_FILE_PATH: &str = "/src/day_1/input.txt";

pub fn solve(replace_chars: bool) {
    let sub_section = if replace_chars {"b"} else {"a"};
    println!("Starting solving day_1 {}.", sub_section);
    let mut tots: u32 = 0;

    let file_path =
        String::from(env::current_dir().unwrap().to_str().unwrap()) + INPUT_FILE_PATH;


    let file = File::open(file_path).unwrap();
    let iterator = io::BufReader::new(file).lines();
    for line in iterator {
        let mut value = line.unwrap();
        if replace_chars {
            value = parse_text_into_digits(value);
        }

        let mut iterator = value.chars().filter(|c| c.is_digit(10));

        let first_char = iterator.nth(0).unwrap();
        let first_digit = first_char.to_digit(10).unwrap();

        let last_digit = iterator.last().unwrap_or(first_char).to_digit(10).unwrap();
        tots += first_digit*10 + last_digit;
    }
    println!("Solved day_1 {}. The solution is {}", sub_section, tots);

}

fn parse_text_into_digits(text: String) -> String{
    return text
        .replace("one", "o1e",)
        .replace("two", "t2o",)
        .replace("three", "t3e",)
        .replace("four", "f4r",)
        .replace("five", "f5e",)
        .replace("six", "s6x",)
        .replace("seven", "s7n",)
        .replace("eight", "e8t",)
        .replace("nine", "n9e",);
}
