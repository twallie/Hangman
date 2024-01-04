use std::fs;
use rand::{thread_rng, Rng};
use std::io::stdin;

fn main() {
    loop {
        let words = read_words();
        let word = get_random_word(&words);
        let word_length = word.len();
        
        let mut hits: u32 = 0;
        let mut misses: u32 = 0;
        let mut used: Vec<char> = vec![];
        const MISSES_ALLOWED: u32 = 6;

        let mut word_frequency_array: Vec<i32> = build_word_frequency_array(&word);
        clear_screen_during_game(word, &word_frequency_array, &used);

        loop {
            if hits == word_length as u32 {
                end_game("🎉 You did it! 🎉", &word);
                break;
            }
            else if misses >= MISSES_ALLOWED {
                end_game("💥 YOU LOST 💥", &word);
                break;
            }

            println!("Guess a letter:");

            let guess = match read_in_alphabetic_character() {
                Ok(v) => {
                    v
                },
                Err(_) => {
                    clear_screen_during_game(word, &word_frequency_array, &used);
                    println!("That's not a lowercase letter!");
                    continue;
                }
            };

            let index = convert_char_to_index(guess);
            if word_frequency_array[index] >= 1 {
                hits += word_frequency_array[index] as u32;
                word_frequency_array[index] = -1;
                clear_screen_during_game(word, &word_frequency_array, &used);  
                println!("That's a character in the word!");
            } else {
                if word_frequency_array[index] == 0 {
                    misses += 1;
                    word_frequency_array[index] = -1;
                    used.push(guess);
                    clear_screen_during_game(word, &word_frequency_array, &used); 
                    println!("Miss!");
                } else {
                    clear_screen_during_game(word, &word_frequency_array, &used); 
                    println!("You already used that letter!");
                }
            }
        }

        println!("Type r to play again");
        match read_in_alphabetic_character() {
            Ok(v) => {
                if v == 'r' {
                    continue;
                } else {
                    return;
                }
            },
            Err(_) => {
                return;
            }
        };
    }
}

fn end_game(message: &str, word: &String) {
    clear_screen();
    println!("{message}");
    println!("The word was {word}");
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn clear_screen_during_game(word: &String, word_frequency_array: &Vec<i32>, used: &Vec<char>) {
    clear_screen();
    for char in word.chars() {
        if word_frequency_array[convert_char_to_index(char)] <= 0 {
            print!("{char}");
        } else {
            print!("_");
        }
    }
    print!("\n");
    
    print!("Not present: ");
    for char in used {
        print!("{char} ");
    }

    print!("\n");
}

fn build_word_frequency_array(word: &String) -> Vec<i32> {
    let mut word_frequency_array: Vec<i32> = vec![0; 26];

    for char in word.chars() {
        let ascii = convert_char_to_index(char);
        word_frequency_array[ascii] += 1;
    }

    word_frequency_array
}

fn convert_char_to_index(char: char) -> usize {
    (char as usize) - ('a' as usize)
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

    match char.is_alphabetic() && char.is_lowercase() {
        true => Ok(char),
        false => return Err("Input was a lowercase letter of the english alphabet")
    }
}

fn get_random_word(words: &Vec<String>) -> &String {
    let random_index = thread_rng().gen_range(0..words.len());
    &words[random_index]
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