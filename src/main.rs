use std::process;

use console::{Style, Term, Color};

mod guess;
mod game;
mod input;

use crate::guess::Guess;
use crate::game::{Game, State};
use crate::input::Input;

fn main() {
    let terminal = Term::stdout();
    let mut game = Game::new();

    loop {
        let input = Input::new(&terminal).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
    
        let guess = Guess::new(input.number);
        
        game = game.guess_number(guess);
        match game.state() {
            State::NoWin(s) => {
                println!("{}", Style::new().bg(Color::Red).apply_to(s));
                println!("{:?}", game.guessed_numbers())
            },
            State::Win(s) => {
                println!("{}", Style::new().bg(Color::Green).apply_to(s));
                process::exit(0);
            },
            State::Starting => println!("Still playing"),
        }
    }
}
