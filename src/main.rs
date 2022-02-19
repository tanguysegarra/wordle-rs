use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use rand::seq::SliceRandom;
use colored::Colorize;

mod error;
use error::Error;

const MAX_ROUNDS: i32 = 6;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let wordlist_path = &args[1];
    let reader = BufReader::new(File::open(wordlist_path)?);
    let words: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let word_to_guess = words.choose(&mut rand::thread_rng()).ok_or(Error::RandFail)?;

    let mut word_try = String::new();

    for _ in 0..MAX_ROUNDS as usize {
        while !words.contains(&word_try.trim_end().to_string()) {
            word_try.clear();
            println!("Enter a valid word: ");
            io::stdin().read_line(&mut word_try)?;
        }
        word_try = word_try.trim_end().to_string();
        println!("+---+---+---+---+---+");
        print!("|");
        for (j, c) in word_try.chars().enumerate() {
            match word_to_guess.find(c) {
                Some(pos) =>
                    if pos == j {
                        print!(" {} |", c.to_string().to_uppercase().black().on_green());
                    } else {
                        print!(" {} |", c.to_string().to_uppercase().black().on_yellow());
                    },
                None => print!(" {} |", c.to_uppercase()),
            }
        }
        print!("\n");
        println!("+---+---+---+---+---+");

        if &word_try == word_to_guess {
            println!("Well played!");
            return Ok(());
        }

        word_try.clear();
    }

    println!("Arf! The word was {}!", word_to_guess);

    Ok(())
}
