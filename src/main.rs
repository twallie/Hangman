mod words;
mod io;

use crate::words::WordFactory;
use crate::io::clear_screen;

static MASTER_WORD_RAW_DATA: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/words.txt"));

fn main() {
    let word_factory = WordFactory::new(MASTER_WORD_RAW_DATA);    
    let word = word_factory.create();
    let word_string = word.as_string();

    clear_screen();
    println!("The generated word is: {}", &word_string);
}
