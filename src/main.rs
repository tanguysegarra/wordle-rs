use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use rand::seq::SliceRandom;

mod error;
use error::Error;

fn get_random_word_from(wordlist_path: String) -> Result<String, Error> {
    let reader = BufReader::new(File::open(wordlist_path)?);
    let words: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    match words.choose(&mut rand::thread_rng()) {
        Some(word) => Ok(word.to_string()),
        None => Err(Error::RandFail),
    }
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let wordlist_path = &args[1];
    let word_to_guess = get_random_word_from(wordlist_path.to_string())?;
    println!("{}", word_to_guess);

    Ok(())
}
