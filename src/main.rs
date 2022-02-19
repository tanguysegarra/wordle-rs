use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};

use rand::seq::SliceRandom;
use colored::Colorize;

mod error;
use error::Error;

const MAX_ROUNDS: i32 = 6;

fn get_random_word_from(wordlist_path: String) -> Result<String, Error> {
    let reader = BufReader::new(File::open(wordlist_path)?);
    let words: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    match words.choose(&mut rand::thread_rng()) {
        Some(word) => Ok(word.to_string()),
        None => Err(Error::RandFail),
    }
}

/** TODO
 *  - [ ] handle double letters
 *  - [ ] refuse words not present in list
 */

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let wordlist_path = &args[1];
    let word_to_guess = get_random_word_from(wordlist_path.to_string())?;

    println!("[DEBUG] Word to guess: {}", word_to_guess);  // TODO remove me

    let mut word_try = String::new();

    for _ in 0..MAX_ROUNDS as usize {
        io::stdin().read_line(&mut word_try)?;
        word_try = word_try.trim_end().to_string();
        let chars = word_try.chars();
        for (j, c) in chars.enumerate() {
            match word_to_guess.find(c) {
                Some(pos) =>
                    if pos == j {
                        print!("{}", c.to_string().to_uppercase().black().on_green());
                    } else {
                        print!("{}", c.to_string().to_uppercase().black().on_yellow());
                    },
                None => print!("{}", c.to_uppercase())
            }
        }
        print!("\n");
        io::stdout().flush()?;

        if word_try == word_to_guess {
            println!("Well played!");
            return Ok(());
        }

        word_try.clear();
    }

    println!("Arf! The word was {}!", word_to_guess);

    Ok(())
}
