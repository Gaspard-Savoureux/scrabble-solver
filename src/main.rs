use std::{io, path::Path};

use dictionnary::Dictionnary;
use docopt::Docopt;

mod dictionnary;

const DEFAULT_FILE_SAVE: &'static str = "dict.bin";
const DEFAULT_WORD_LIST: &'static str = "wordlists/words_alpha.txt";
const USAGE: &'static str = "
Usage:
    scrabble-solver [options] [INPUT]
    scrabble-solver (--help)

Options:
    -h, --help               Show this message.
    -l, --load INPUTFILE     Load existing dictionnary from a text json or bin file.
    -o, --output OUTPUTFILE  Save the dictionnary to a given file. Only support .json and .bin.
    --json-format            Format the saved file to JSON format instead of binary code.
";

fn main() {
    let argv = std::env::args();

    let args = Docopt::new(USAGE)
        .and_then(|d| d.argv(argv.into_iter()).parse())
        .unwrap_or_else(|e| e.exit());

    // DEBUG
    // println!("{:?}", args);

    let mut dict = Dictionnary::new();

    // Loading files
    let default_save_exists = Path::new(DEFAULT_FILE_SAVE).exists();
    let wordlist = args.get_str("-l");

    // Load dictionnary
    match (wordlist, default_save_exists) {
        ("", true) => {
            dict.load_bin_file(&DEFAULT_FILE_SAVE);
        }
        ("", false) => {
            dict.load_words_from_file(&DEFAULT_WORD_LIST);
            dict.save_to_bin_file(&DEFAULT_FILE_SAVE);
        }
        (wordlist, _) => {
            if wordlist.contains(".json") {
                dict.load_json_file(wordlist)
            } else if wordlist.contains(".bin") {
                dict.load_bin_file(wordlist);
            } else {
                dict.load_words_from_file(wordlist);
            }
        }
    }

    // Save dictionnary to file, default -> bin
    let output_file: &str = if args.get_bool("-o") {
        args.get_str("-o")
    } else {
        DEFAULT_FILE_SAVE
    };

    println!("OUTPUT_FILE: {}", output_file);
    if output_file.contains(".json") || args.get_bool("--json-format") {
        dict.save_to_json_file(output_file);
    } else {
        dict.save_to_bin_file(output_file);
    }

    // Input handling
    let letters: Vec<char> = args.get_str("INPUT").to_uppercase().chars().collect();

    if letters.len() > 0 {
        let mut words = dict.get_words(letters);
        words.sort_by(|a, b| a.len().cmp(&b.len()));

        for word in words {
            println!("{}", word);
        }
    } else {
        let mut input_text = String::new();
        loop {
            println!("Enter your letters: ");

            io::stdin()
                .read_line(&mut input_text)
                .expect("failed to read from stdin");

            println!();

            let mut words = dict.get_words(input_text.to_uppercase().chars().collect());
            words.sort_by(|a, b| a.len().cmp(&b.len()));
            for word in words {
                if word.len() > 1 {
                    println!("{}", word);
                }
            }
            input_text.clear();
        }
    }
}
