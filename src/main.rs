use std::ops::RangeInclusive;
use crate::game::game::{Difficulty, Game};
use crate::io::io::{print, read_string};
use colored::Colorize;

mod game;
mod io;

const MAX: i32 = 999;
const MIN: i32 = 100;

fn main() {
    let diffs = Vec::from([
        Difficulty::new("Easy", 15),
        Difficulty::new("Medium", 10),
        Difficulty::new("Hard", 6),
        Difficulty::new("Fortune", 1)
    ]);

    loop {
        print("\nEnter your name >> ");
        let name = read_string();

        println!();

        let mut game = Game::init(diffs.clone(), RangeInclusive::new(MIN, MAX), name.clone());

        let results = game.start();

        let res_text = match results.win {
            true => "You guessed!".green().bold(),
            false => "You have no more attempts. You loose..".red().bold()
        };

        println!("{res_text}");

        if results.win {
            println!("Attempts spent: {}", game.diff.attempts - results.att_wasted);
        } else {
            println!("Password was {}, but your closest guess is {}. You wasted {} attempt(s) on this!", results.pass, results.best_guess, results.att_wasted);
        }

        print("\nWanna play again? y/n >> ");
        let input = read_string();

        if input == "n" {
            println!("Goodbye, {}", name.clone());
            break
        }

        if input == "y" {
            println!("Here were go again!");
            continue
        }
    }
}