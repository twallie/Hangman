use std::fs;
use rand::{thread_rng, Rng};

fn main() {
    let words = read_words();
    let random_word = get_random_word(&words);
    println!("{random_word}");
}

fn get_random_word(words: &Vec<String>) -> &String {
    let random_index = thread_rng().gen_range(0..words.len());
    return &words[random_index];
}

fn read_words() -> Vec<String> {
    const FILEPATH: &str = "resources/words.txt";
    let words: Vec<String> = fs::read_to_string(FILEPATH)
        .expect("Couldn't read the file")
        .lines()
        .map(|line| {
            line.split_whitespace().collect()
        }).collect();
    return words;
}