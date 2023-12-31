use std::{fs, collections::HashSet};
use rand::{thread_rng, Rng};
use std::io::stdin;

fn main() {
    let words = read_words();
    let word = get_random_word(&words);
    let hashset = build_word_hashset(word);

    loop {
        println!("Guess a letter:");

        let guess = match read_in_alphabetic_character() {
            Ok(v) => v,
            Err(_) => continue
        };

        println!("That do be what I am looking for fr fr");
    }
}

fn read_in_alphabetic_character() -> Result<char, &'static str> {
    let mut guess: String = String::new();
    let stdin_result = stdin()
        .read_line(&mut guess);

    match stdin_result {
        Ok(_) => (),
        Err(_) => return Err("Error reading from stdin")
    };

    let char: char = match guess.trim().parse() {
        Ok(v) => v,
        Err(_) => return Err("Error converting user input, likely not a char"),
    };

    match char.is_alphabetic() {
        true => return Ok(char),
        false => return Err("Input was not alphabetic")
    }
}

fn build_word_hashset(word: &String) -> HashSet<char> {
    let mut hashset: HashSet<char> = HashSet::new();
    for char in word.chars() {
        hashset.insert(char);
    }
    return hashset;
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