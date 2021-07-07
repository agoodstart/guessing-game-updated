use rand::Rng;
use std::cmp::Ordering;

use crate::guess::Guess;

pub struct Game {
    secret_number: i32,
    guessed_numbers: Vec<i32>,
    state: State<&'static str>,
}

#[derive(Debug, Clone, Copy)]
pub enum State<T> {
    Starting,
    Win(T),
    NoWin(T),
}

impl Game {
    pub fn new() -> Game {
        Game {
            secret_number: rand::thread_rng().gen_range(1..101),
            guessed_numbers: Vec::new(),
            state: State::Starting,

        }
    }

    pub fn number(&self) -> i32 {
        self.secret_number
    }

    pub fn guessed_numbers(&self) -> &Vec<i32> {
        &self.guessed_numbers
    }

    pub fn state(&self) -> State<&str> {
        self.state
    }

    pub fn guess_number(mut self, guess: Guess) -> Self {
        println!("You guessed {}", guess.value());

        match guess.value().cmp(&self.secret_number) {
            Ordering::Less => self.state = State::NoWin(" Too Small! Try again: "),
            Ordering::Greater => self.state = State::NoWin(" Too big! Try again: "),
            Ordering::Equal => self.state = State::Win(" You win! "),
        }

        self.guessed_numbers.push(guess.value());

        self
    }
}