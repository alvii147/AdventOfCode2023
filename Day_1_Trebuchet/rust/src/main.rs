use lazy_static::lazy_static;
use std::{collections::HashMap, fs};

fn read_file(filename: &str) -> String {
    let file_contents: String =
        fs::read_to_string(filename).expect("Should have been able to read file");

    return file_contents;
}

fn calibrate_digits(line: &str) -> Result<u32, &str> {
    let mut left_digit: Option<u32> = None;
    for c in line.chars() {
        if c.is_digit(10) {
            left_digit = c.to_digit(10);
            break;
        }
    }

    let mut right_digit: Option<u32> = None;
    for c in line.chars().rev() {
        if c.is_digit(10) {
            right_digit = c.to_digit(10);
            break;
        }
    }

    let mut cal_val: Result<u32, &str> = Err("Failed to calibrate");
    if let (Some(l), Some(r)) = (left_digit, right_digit) {
        cal_val = Ok((l * 10) + r);
    }

    return cal_val;
}

fn find_word(line: &str, words: &Vec<&str>, reverse: bool) -> Option<usize> {
    let mut first_char_idx: Option<usize> = None;
    let mut first_word_idx: Option<usize> = None;

    for (word_idx, word) in words.iter().enumerate() {
        let search: Option<usize>;
        if reverse {
            search = line.rfind(word)
        } else {
            search = line.find(word)
        }

        if let Some(char_idx) = search {
            if let Some(curr_char_idx) = first_char_idx {
                if (reverse && char_idx > curr_char_idx) || (!reverse && char_idx < curr_char_idx) {
                    first_char_idx = Some(char_idx);
                    first_word_idx = Some(word_idx);
                }
            } else {
                first_char_idx = Some(char_idx);
                first_word_idx = Some(word_idx);
            }
        }
    }

    return first_word_idx;
}

fn calibrate_words(line: &str) -> Result<u32, &str> {
    lazy_static! {
        static ref NUM_MAP: HashMap<&'static str, u32> = HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
        ]);
    }

    let mut num_keys = Vec::new();
    for k in NUM_MAP.keys() {
        num_keys.push(*k);
    }

    let mut cal_val: Result<u32, &str> = Err("Failed to calibrate");

    if let (Some(left_word_idx), Some(right_word_idx)) = (
        find_word(line, &num_keys, false),
        find_word(line, &num_keys, true),
    ) {
        if let (Some(l), Some(r)) = (
            NUM_MAP.get(num_keys[left_word_idx]),
            NUM_MAP.get(num_keys[right_word_idx]),
        ) {
            cal_val = Ok(((*l) * 10) + (*r));
        }
    }

    return cal_val;
}

fn main() {
    let filename: &str = "../document.txt";
    let file_contents: String = read_file(filename);

    let mut cal_digits_sum: u32 = 0;
    let mut cal_words_sum: u32 = 0;
    for line in file_contents.lines() {
        if let Ok(val) = calibrate_digits(line) {
            cal_digits_sum += val;
        }

        if let Ok(val) = calibrate_words(line) {
            cal_words_sum += val;
        }
    }

    println!("{}", cal_digits_sum);
    println!("{}", cal_words_sum);
}
