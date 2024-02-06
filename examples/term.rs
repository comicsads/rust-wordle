use std::io;

use rand::prelude::*;
use wordle_lib::Guess;

enum GameVictory {
    Won,
    Lost,
}

impl std::fmt::Display for GameVictory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match *self {
            GameVictory::Won => "You won!",
            GameVictory::Lost => "You lost :c",
        };

        write!(f, "{text}")
    }
}

fn main() {
    let words = include_str!("words");
    let dict = words.split_whitespace();
    let answer = dict
        .choose(&mut rand::thread_rng())
        .expect("dict shouldn't be empty");

    let answer = Guess::build(answer.to_owned()).unwrap();
    // let answer = unsafe { Guess::new(answer.to_owned()) }; // Alternative option

    #[cfg(debug_assertions)] // let us cheat while developing
    println!("answer: {answer}");

    let did_win = play_game(&answer);
    println!("{did_win}")
}

fn play_game(answer: &Guess) -> GameVictory {
    for i in 0..6 {
        let user_guess = get_any_guess();
        let resp = user_guess.verify(&answer);

        println!("{}", resp.unpretty_string());
        println!("{} guesses left!", 5 - i);

        if resp.victory() {
            return GameVictory::Won;
        }
    }
    println!("answer was {answer}");
    GameVictory::Lost
}

/// Will loop until valid guess achieved
fn get_any_guess() -> Guess {
    let mut user_guess: Option<Guess> = None;
    while user_guess.is_none() {
        user_guess = manage_user_guess();
    }
    user_guess.expect("impossible to be none after while loop")
}

/// If there's an error getting the guess it will return a None and print out the error
fn manage_user_guess() -> Option<Guess> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let guess_maybe = Guess::build(input.trim().to_owned());

    match guess_maybe {
        Ok(guess) => Some(guess),
        Err(e) => {
            println!("Couldn't parse your guess: {e}");
            None
        }
    }
}
