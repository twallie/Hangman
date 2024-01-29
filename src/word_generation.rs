use std::fs;

use rand::{thread_rng, Rng};    

const FILEPATH: &str = "/home/tom/.homebrewed/hangman/resources/words.txt";

pub fn get_random_word(words: &Vec<String>) -> &String {
    let random_index = thread_rng().gen_range(0..words.len());
    &words[random_index]
}

pub fn read_words() -> Vec<String> {
    let words: Vec<String> = fs::read_to_string(FILEPATH)
        .expect("Couldn't read the file")
        .lines()
        .map(|line| {
            line.split_whitespace().collect()
        }).collect();
    return words;
}