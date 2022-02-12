use std::fs::File;
use std::io::{self, BufRead};
use std::iter::FromIterator;

fn suggest_word(word: &String) {
    println!("enter the word: {}", word);
}

fn apply_response_filter(
    words: Vec<String>,
    suggested_word: String,
    response_pattern: String,
) -> Vec<String> {
    let mut filtered_words: Vec<String> = words;

    for n in 0..4 {
        let word_char = suggested_word.chars().nth(n).unwrap();
        let response_char = response_pattern
            .chars()
            .nth(n)
            .unwrap()
            .to_digit(10)
            .unwrap();

        match response_char {
            2 => {
                println!("found char {} at pos {}", word_char, n);
                filtered_words = filtered_words
                    .into_iter()
                    .filter(|word| word.chars().nth(n) == Some(word_char))
                    .collect()
            }
            1 => {
                println!("char {} is inside the word somewhere", word_char);
                filtered_words = filtered_words
                    .into_iter()
                    .filter(|word| word.contains(word_char))
                    .collect()
            }
            0 => {
                println!("char {} is not in  the word", word_char);
                filtered_words = filtered_words
                    .into_iter()
                    .filter(|word| !word.contains(word_char))
                    .collect()
            }
            _ => {}
        }
    }

    filtered_words
}

fn main() {
    println!("Hello, world!");

    let file = File::open("words.txt").unwrap();
    let words = Vec::from_iter(io::BufReader::new(file).lines().map(|word| word.unwrap()));

    let suggested_word = "marry".to_string();
    suggest_word(&suggested_word);

    let response_pattern = "01120".to_string();

    let filtered_words = apply_response_filter(words, suggested_word, response_pattern);

    for n in 0..10 {
        match filtered_words.get(n) {
            Some(word) => println!("{}: {}", n, word),
            None => break,
        }
    }
}
