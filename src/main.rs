mod words;

use crate::words::WordFactory;

static MASTER_WORD_RAW_DATA: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/words.txt"));

fn main() {
    let word_factory = WordFactory::new(MASTER_WORD_RAW_DATA);    
    let mut count = 0;

    while count < 5 {
        let word = word_factory.create();
        println!("{}", word.as_string());
        count += 1;
    }
}
