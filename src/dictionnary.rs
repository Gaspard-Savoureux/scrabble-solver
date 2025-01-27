use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter},
    str::Chars,
};

#[derive(Debug, Serialize, Deserialize)]
struct Node {
    end_of_word: bool,
    children: HashMap<char, Node>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            end_of_word: false,
            children: HashMap::with_capacity(26),
        }
    }

    pub fn insert(&mut self, word: Chars) {
        let mut node = self;

        for c in word {
            match node.children.get(&c) {
                Some(_) => node = node.children.get_mut(&c).unwrap(),
                None => {
                    node.children.insert(c, Node::new());
                    node = node.children.get_mut(&c).unwrap();
                }
            }
        }

        node.end_of_word = true
    }

    pub fn get_words(&self, current_word: Vec<char>, letters: Vec<char>) -> Vec<Vec<char>> {
        let mut words: Vec<Vec<char>> = Vec::new();

        let mut duplicate_letters: Vec<char> = Vec::new();

        let node = self;
        for (index, letter) in letters.iter().enumerate() {
            if duplicate_letters.contains(&letter) {
                continue;
            }

            if let Some(child) = node.children.get(&letter) {
                // New existing combination to explore
                let mut new_word = current_word.clone();
                new_word.push(*letter);

                let mut remaining_letters = letters.clone();
                remaining_letters.remove(index);

                // If current combination is a word, we keep
                if child.end_of_word {
                    words.push(new_word.clone());
                }

                words.extend(child.get_words(new_word, remaining_letters));
            }

            duplicate_letters.push(*letter);
        }

        words
    }
}

pub struct Dictionnary {
    root: Node,
}

impl Dictionnary {
    pub fn new() -> Dictionnary {
        Dictionnary { root: Node::new() }
    }

    pub fn add_word(&mut self, word: Chars) {
        self.root.insert(word);
    }

    pub fn get_words(&self, letters: Vec<char>) -> Vec<String> {
        let words = self.root.get_words(Vec::new(), letters);

        let mut concated_word: Vec<String> = Vec::new();
        for word in words {
            concated_word.push(word.into_iter().collect());
        }

        concated_word
    }

    /// Save the current dict to a file in JSON format
    ///
    /// Slower then save [`save_to_bin_file`], but human readable
    pub fn save_to_json_file(&self, filepath: String) {
        let file = File::create(filepath).expect("Failed to create file");
        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, &self.root).expect("Failed to read JSON");
    }

    /// Load a file with a dictionnary save to the JSON format
    pub fn load_json_file(&mut self, filepath: String) {
        let file = File::open(filepath).expect("Failed to open file");
        let reader = BufReader::new(file);

        let new_root: Node = serde_json::from_reader(reader).expect("Failed to read JSON");
        self.root = new_root;
    }

    pub fn save_to_bin_file(&self, filepath: String) {
        !todo!()
    }

    pub fn load_bin_file(&mut self, filepath: String) {
        !todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_context_test() {
        let mut dict = Dictionnary::new();
        dict.add_word("RUST".chars());
        dict.add_word("ALLIGATOR".chars());
        dict.add_word("RUDIMENTAIRE".chars());
        dict.add_word("RUSE".chars());

        let letters_in_players_hand = vec![
            'A', 'S', 'U', 'T', 'R', 'E', 'D', 'I', 'M', 'N', 'T', 'I', 'R', 'E',
        ];

        let words = dict.get_words(letters_in_players_hand);

        assert!(words.contains(&String::from("RUST")));
        assert!(words.contains(&String::from("RUDIMENTAIRE")));
        assert!(words.contains(&String::from("RUSE")));
    }

    #[test]
    fn save_and_load_json_file_test() {
        let filename: String = "test.json".to_string();

        let mut dict1 = Dictionnary::new();
        dict1.add_word("RUST".chars());
        dict1.add_word("ALLIGATOR".chars());
        dict1.add_word("RUDIMENTAIRE".chars());
        dict1.add_word("RUSE".chars());
        dict1.save_to_json_file(filename);

        let mut dict2 = Dictionnary::new();
        dict2.load_json_file("data.json".to_string());

        let letters_in_players_hand = vec![
            'A', 'S', 'U', 'T', 'R', 'E', 'D', 'I', 'M', 'N', 'T', 'I', 'R', 'E',
        ];

        let words = dict2.get_words(letters_in_players_hand);

        assert!(words.contains(&String::from("RUST")));
        assert!(words.contains(&String::from("RUDIMENTAIRE")));
        assert!(words.contains(&String::from("RUSE")));
    }

    #[test]
    fn save_and_load_bin_file_test() {}
}
