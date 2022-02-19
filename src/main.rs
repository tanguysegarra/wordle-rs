use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use rand::seq::SliceRandom;

mod error;
use error::Error;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let wordlist = &args[1];
    let reader = BufReader::new(File::open(wordlist)?);

    let words: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let word_to_guess = match words.choose(&mut rand::thread_rng()) {
        Some(word) => word,
        None => return Err(Error::RandFail)
    };

    println!("{}", word_to_guess);

    Ok(())
}
