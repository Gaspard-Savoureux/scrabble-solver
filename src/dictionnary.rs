use std::{collections::HashMap, str::Chars};

#[derive(Debug)]
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
}
