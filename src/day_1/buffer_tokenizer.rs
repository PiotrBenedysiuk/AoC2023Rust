use std::collections::VecDeque;
use std::{env, io};
use std::fs::File;
use std::io::BufRead;
use std::str::Chars;

const TOKENS_A: [Token; 18] =[
    Token { key: "1", value: 1 },
    Token { key: "2", value: 2 },
    Token { key: "3", value: 3 },
    Token { key: "4", value: 4 },
    Token { key: "5", value: 5 },
    Token { key: "6", value: 6 },
    Token { key: "7", value: 7 },
    Token { key: "8", value: 8 },
    Token { key: "9", value: 9 },
    Token { key: "1", value: 1 },
    Token { key: "2", value: 2 },
    Token { key: "3", value: 3 },
    Token { key: "4", value: 4 },
    Token { key: "5", value: 5 },
    Token { key: "6", value: 6 },
    Token { key: "7", value: 7 },
    Token { key: "8", value: 8 },
    Token { key: "9", value: 9 },
];

const TOKENS_B: [Token; 18] =[
    Token { key: "1", value: 1 },
    Token { key: "2", value: 2 },
    Token { key: "3", value: 3 },
    Token { key: "4", value: 4 },
    Token { key: "5", value: 5 },
    Token { key: "6", value: 6 },
    Token { key: "7", value: 7 },
    Token { key: "8", value: 8 },
    Token { key: "9", value: 9 },
    Token { key: "one", value: 1 },
    Token { key: "two", value: 2 },
    Token { key: "three", value: 3 },
    Token { key: "four", value: 4 },
    Token { key: "five", value: 5 },
    Token { key: "six", value: 6 },
    Token { key: "seven", value: 7 },
    Token { key: "eight", value: 8 },
    Token { key: "nine", value: 9 },
];

struct SizeLimitedBuffer {
    buffer_size: usize,
    values: VecDeque<char>
}

trait Buffer {
    fn append(&mut self, other: char);
    fn try_remove(&mut self, value: &str) -> bool;
}

impl Buffer for SizeLimitedBuffer {
    fn append(&mut self, other: char) {
        if self.values.len() == self.buffer_size{
            self.values.pop_front();
        }
        self.values.push_back(other);
    }
    fn try_remove(&mut self, value: &str) -> bool {
        if self.values.is_empty() { return false }
        let mut word: String = self.values.iter().collect();
        if word.contains(value) {
            word = word.replace(value, "");
            self.values.clear();
            for char in word.chars() {
                self.values.push_back(char);
            }
            return true
        }
        return false
    }
}

struct Token<'a> {
    key: &'a str,
    value: i32
}
struct TokenCollection<'a> {
    tokens: [Token<'a>; 18],
    buffer: SizeLimitedBuffer
}

fn get_token_collection(tokens: [Token; 18]) -> TokenCollection {
    let buffer_size = tokens.map(|token| token.key.len()).iter().max().unwrap() + 1;
    return TokenCollection {
        tokens: tokens,
        buffer: SizeLimitedBuffer {
            buffer_size,
            values: VecDeque::new()
        }
    }
}


trait FindTokens {
    fn next(self, characters: Chars<'_>) -> Option<(i32, Chars)>;
    fn last(self, characters: Chars<'_>) -> Option<i32>;
}

impl FindTokens for TokenCollection<'_> {
    fn next(mut self, mut characters: Chars<'_>) -> Option<(i32, Chars)> {
        let mut val: Option<i32> = None;
        for character in characters.by_ref() {
            self.buffer.append(character);
            for token in &self.tokens {
                if self.buffer.try_remove(&token.key) { val = Some(token.value); break; }
            }
            if val.is_some() { break; }
        }
        if val.is_some() {return Some((val.unwrap(), characters));}
        return None
    }
    fn last(mut self, characters: Chars<'_>) -> Option<i32> {
        let mut value: i32;
        let mut iterator = characters;
        let mut next_value = self.next(iterator);

        if next_value.is_none() { return None}
        while next_value.is_some() {
            (value, iterator)=next_value.unwrap();
            next_value=self.next(iterator);
        }
        return Some(value);
    }
}

const INPUT_FILE_PATH: &str = "/src/day_1/input.txt";
pub fn better_solve(replace_chars: bool) {
    let sub_section = if replace_chars {"b"} else {"a"};
    println!("Starting solving day_1 {}.", sub_section);
    let file_path =
        String::from(env::current_dir().unwrap().to_str().unwrap()) + INPUT_FILE_PATH;

    let file = File::open(file_path).unwrap();
    let iterator = io::BufReader::new(file).lines();
    let mut tots = 0;
    for line in iterator {

        println!("went thru line!");
        let value = line.unwrap();

        println!("getting tokenizer");
        let mut tokenizer = get_token_collection(if replace_chars { TOKENS_B } else { TOKENS_A });


        println!("getting first");
        let (first_digit, iterator) = tokenizer.next(value.chars()).unwrap();

        println!("getting last");
        let last_digit = tokenizer.last(iterator).unwrap_or(first_digit);

        tots += first_digit * 10 + last_digit
    }
    println!("Solved day_1 {}. The solution is {}", sub_section, tots);
}

