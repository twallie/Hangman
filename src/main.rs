use std::fs;

fn main() {
    const FILEPATH: &str = "resources/words.txt";
    let contents = fs::read_to_string(FILEPATH).expect("Couldn't read the file");    
}
