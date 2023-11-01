use std::ops::{RangeInclusive};
use std::process::exit;
use rand::Rng;
use colored::Colorize;
use crate::io::io::{print, read_num};

#[derive(Clone)]
pub struct Difficulty {
    pub title: String,
    pub attempts: u8
}

impl Difficulty {
    pub fn new(title: &str, attempts: u8) -> Difficulty {
        return Difficulty {
            title: title.to_string(),
            attempts
        }
    }
}

pub struct GameResults {
    pub win: bool,
    pub pass: i32,
    pub att_wasted: u8,
    pub best_guess: i32
}

pub struct Game {
    pub range: RangeInclusive<i32>,
    pub diff: Difficulty,
    pass: i32,
    att_left: u8
}

impl Game {
    pub fn init(diffs: Vec<Difficulty>, range: RangeInclusive<i32>, player: String) -> Game {
        println!("Hello, {player}! In this game you'll need to guess the number from {} to {}.", range.clone().min().unwrap(),
                                                                                                 range.clone().max().unwrap());

        let diff = Game::select_diff(diffs);
        println!("Now let's select a difficulty to play.");


        println!("You selected '{}' difficulty.", diff.title);

        let pass = Game::gen_pass(range.clone());

        return Game {
            range,
            diff: diff.clone(),
            pass,
            att_left: diff.attempts
        };
    }

    pub fn start(&mut self) -> GameResults {
        let mut best: i32 = self.range.clone().max().unwrap() * 2;

        loop {
            println!();

            print("Enter a number >> ");
            let input = read_num();

            // Check if input is even a number
            if input.is_none() {
                println!("Not a number!");
                if !&self.update_att() { break }
                continue
            }

            let guess = input.unwrap();

            // Check if player's input is in acceptable range
            if !&self.range.contains(&guess) {
                println!("You number is outside of a given range. Try again");
                if !&self.update_att() { break }
                continue
            }

            // Update closest guess
            best = if (best - self.pass).abs() > (guess - self.pass).abs() {
                guess
            } else {
                best
            };

            // If player already lost all his attempts or has guessed the pass - exit a loop
            if !self.update_att() || guess == self.pass {
                break
            }

            // If player didn't guessed a password, tell him whats wrong
            let text = match () {
                _ if guess <  self.pass => "Password is greater! Try again".magenta(),
                _ if guess >  self.pass => "Password is less! Try again".bright_blue(),
                _ => "wtf".bold().red().underline()
            };

            println!("{}", text);
        }

        println!();

        return GameResults {
            win: best == self.pass,
            pass: self.pass,
            att_wasted: self.diff.attempts - self.att_left,
            best_guess: best
        };
    }

    fn update_att(&mut self) -> bool {
        if self.att_left == 0 {
            return false;
        }

        self.att_left -= 1;

        println!("You have {} more attempts", self.att_left);

        return true;
    }

    fn gen_pass(range: RangeInclusive<i32>) -> i32 {
        let mut rng = rand::thread_rng();

        return rng.gen_range(range);
    }

    fn select_diff(diffs: Vec<Difficulty>) -> Difficulty {
        let mut i = 0;

        println!("\nAvailable difficulties:");

        for diff in diffs.iter() {
            println!("{i}. {} - {} attempts", diff.title, diff.attempts);
            i += 1;
        };

        print("Select >> ");

        let choose = read_num().unwrap() as usize;

        if  choose > diffs.len() {
            println!("Wrong index");
            exit(0);
        }

        return  diffs.get(choose).unwrap().clone();
    }
}