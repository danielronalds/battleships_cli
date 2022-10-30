mod grid;
mod battleship;

use crate::grid::Grid;
use crate::battleship::Battleship;
use crate::battleship::Direction;

use std::io;
use std::io::Write;

fn main() {
    let mut battleships: Vec<Battleship> = Vec::new();

    battleships.push(Battleship::new(0, 0, 2, Direction::Horizontal));
    battleships.push(Battleship::new(3, 0, 3, Direction::Vertical));

    let grid = Grid::build(5,5).unwrap();

    while !game_over() {
        grid.draw(&battleships);

        // Getting the x and y coordinates
        print!("Please type an x coordinate: ");
        io::stdout().flush().unwrap();
        let x_guess = get_guess();

        print!("Please type an y coordinate: ");
        io::stdout().flush().unwrap();
        let y_guess = get_guess();

        println!("");
    }
}

fn game_over() -> bool {
    false
}

fn get_guess() -> u8 {
    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to readline");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("Invalid coordinate! Try again: ");
                io::stdout().flush().unwrap();
                continue;
            }
        };

        return guess;
    }
}
