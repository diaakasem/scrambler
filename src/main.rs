// use clap::{App, Arg};
use rand::{thread_rng, Rng};
use regex::Regex;
use std::io::Read;

fn scramble_text(text: &str, untouchable: &Vec<Regex>) -> String {
    let mut result = String::new();
    let lines = text.split('\n');
    let mut lines_iter = lines.into_iter();
    while let Some(line) = lines_iter.next() {
        let mut line_scrambled = String::new();
        let leading_whitespaces = line.chars().take_while(|c| c.is_whitespace()).count();
        for word in line.trim().split_whitespace() {
            let mut scrambled_word = String::new();
            let mut skip_scrambling = false;
            for untouchable_regex in untouchable {
                if untouchable_regex.is_match(word) {
                    skip_scrambling = true;
                    break;
                }
            }
            if skip_scrambling {
                scrambled_word = word.to_owned();
            } else {
                let mut rng = thread_rng();
                for c in word.chars() {
                    if c.is_ascii_uppercase() {
                        scrambled_word.push(scramble_uppercase(c, &mut rng));
                    } else if c.is_ascii_lowercase() {
                        scrambled_word.push(scramble_lowercase(c, &mut rng));
                    } else if c.is_numeric() {
                        scrambled_word.push(scramble_digit(c, &mut rng));
                    } else {
                        scrambled_word.push(c);
                    }
                }
            }
            line_scrambled.push_str(&scrambled_word);
            line_scrambled.push(' ');
        }
        result.push_str(&" ".repeat(leading_whitespaces));
        result.push_str(&line_scrambled.trim());
        if lines_iter.next().is_some() {
            result.push('\n');
        }
    }
    result
}

fn scramble_uppercase(c: char, rng: &mut impl Rng) -> char {
    let base = b'A' as u8;
    let range = 26;
    (base + (c as u8 - base + rng.gen_range(1..=range)) % range) as char
}

fn scramble_lowercase(c: char, rng: &mut impl Rng) -> char {
    let base = b'a' as u8;
    let range = 26;
    (base + (c as u8 - base + rng.gen_range(1..=range)) % range) as char
}

fn scramble_digit(c: char, rng: &mut impl Rng) -> char {
    let base = b'0' as u8;
    let range = 10;
    (base + (c as u8 - base + rng.gen_range(1..=range)) % range) as char
}

fn main() {
    // let matches = App::new("text-scrambler")
    //     .arg(Arg::with_name("text").required(true).takes_value(true))
    //     .arg(Arg::with_name("untouchable").multiple(true).takes_value(true))
    //     .get_matches();
    //
    let mut text = String::new();
    std::io::stdin().read_to_string(&mut text).unwrap();
    let untouchable: Vec<Regex> = vec![];

    let scrambled = scramble_text(text.as_str(), &untouchable);
    println!("{}", scrambled);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scramble_text() {
        let text = "hello world";
        let untouchable = vec![Regex::new("hello").unwrap()];
        let scrambled = scramble_text(text, &untouchable);
        let scrambled_words: Vec<&str> = scrambled.split(" ").collect();
        assert_eq!(scrambled_words[0], "hello");
        assert_ne!(scrambled_words[1], "world");
    }

    #[test]
    fn test_scramble_uppercase() {
        let c = 'A';
        let mut rng = thread_rng();
        let scrambled = scramble_uppercase(c, &mut rng);
        assert!(scrambled >= 'A' && scrambled <= 'Z');
    }

    #[test]
    fn test_scramble_lowercase() {
        let c = 'a';
        let mut rng = thread_rng();
        let scrambled = scramble_lowercase(c, &mut rng);
        assert!(scrambled >= 'a' && scrambled <= 'z');
    }

    #[test]
    fn test_scramble_digit() {
        let c = '0';
        let mut rng = thread_rng();
        let scrambled = scramble_digit(c, &mut rng);
        assert!(scrambled >= '0' && scrambled <= '9');
    }
}
