use std::fs::File;
use std::io::{self, stdin, BufRead};
use std::iter::FromIterator;

fn suggest_word(word: &String) {
    println!("enter the word: {}", word);
}

fn apply_response_filter(
    words: Vec<String>,
    suggested_word: &String,
    response_pattern: &String,
) -> Vec<String> {
    let mut filtered_words: Vec<String> = words;
    let mut chars_in_word: Vec<char> = Vec::new();

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
                chars_in_word.push(word_char);
            }
            1 => {
                chars_in_word.push(word_char);
            }

            _ => {}
        }
    }

    for known_char in &chars_in_word {
        println!("already know character {}", known_char)
    }

    for n in 0..5 {
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
                if !chars_in_word.contains(&word_char) {
                    println!("char {} is not in  the word", word_char);

                    filtered_words = filtered_words
                        .into_iter()
                        .filter(|word| !word.contains(word_char))
                        .collect()
                } else {
                    println!("is somewhere else aswell {}", word_char)
                }
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

    let mut suggested_word = "aesir".to_string();
    let mut response_pattern = String::new();
    let mut filtered_words = words;

    let mut attempts = 1;

    while attempts < 6 {
        suggest_word(&suggested_word);

        response_pattern.clear();

        stdin()
            .read_line(&mut response_pattern)
            .expect("enter a valid string");

        println!("you entered the pattern {}", response_pattern);

        if response_pattern.len() < 5 {
            println!("pattern needs to be 5 chars long");
            continue;
        }

        response_pattern = response_pattern[..5].to_string();

        filtered_words = apply_response_filter(filtered_words, &suggested_word, &response_pattern);

        for n in 0..10 {
            match filtered_words.get(n) {
                Some(word) => println!("{}: {}", n, word),
                None => println!("could not find word for index {}", n),
            }
        }

        println!("enter the next world you would like to try");

        let mut entered_word = String::new();
        stdin()
            .read_line(&mut entered_word)
            .expect("enter a valid string");

        println!("you entered the pattern {}", entered_word);

        if entered_word.len() < 5 {
            println!("word needs to be 5 chars long");
            continue;
        }

        suggested_word.clear();
        suggested_word = entered_word[..5].to_string();

        attempts += 1;
    }
}
