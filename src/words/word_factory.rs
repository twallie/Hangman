use rand::seq::SliceRandom;

pub struct WordFactory {
    word_list: Vec<String>,
}

impl WordFactory {
    pub fn new(newline_separated_str: &str) -> WordFactory {
        return WordFactory {
            word_list: parse_data_into_vec(newline_separated_str)
        }
    }

    pub fn create(&self) -> Word {
        Word::new(self.word_list.choose(&mut rand::thread_rng()).unwrap())
    }
}

fn parse_data_into_vec(data: &str) -> Vec<String> {
    data.split("\n").map(|s| s.to_string().to_owned()).collect()
}

pub struct Word {
    string: String
}

impl Word {
    pub fn new(value: &str) -> Word {
        Word {
            string: value.to_string()
        }
    }

    pub fn as_string(&self) -> String {
        self.string.clone()
    }
}