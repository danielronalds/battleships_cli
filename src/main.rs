mod grid;
mod point;
mod battleship;

use crate::grid::Grid;
use crate::battleship::Battleship;
use crate::battleship::Direction;
use crate::point::Point;

use std::io;
use std::io::Write;

fn main() {
    // Creating battleships
    let mut battleships: Vec<Battleship> = Vec::new();

    battleships.push(Battleship::new(0, 0, 2, Direction::Horizontal));
    battleships.push(Battleship::new(3, 0, 3, Direction::Vertical));

    // Creating the grid
    let grid = Grid::build(5,5).unwrap();

    // vecs for storing hits and misses
    let mut misses: Vec<Point> = Vec::new();
    let mut hits: Vec<Point> = Vec::new();

    while !game_over(&battleships, &hits) {
        // Drawing the grid
        // grid.draw(&battleships);
        grid.draw_hits_and_misses(&hits, &misses);

        println!("");

        // Getting the x and y coordinates
        println!("Please pick an X and Y position");
        let x_guess = get_guess("X", grid.width() - 1);

        let y_guess = get_guess("Y", grid.height() - 1);

        // Checking for hits
        let mut hit = false;

        for battleship in &battleships {
            if battleship.occupies(x_guess, y_guess) {
                hit = true;
                break;
            }
        }

        // Clearing the screen
        print!("{esc}c", esc = 27 as char);

        if hit {
            println!("That was a hit!");
            hits.push(Point::new(x_guess, y_guess));
        } else {
            misses.push(Point::new(x_guess, y_guess));
            println!("That was a miss!");
        }
    }

    println!("You won!");
}

fn game_over(battleships: &Vec<Battleship>, hits: &Vec<Point>) -> bool {
    let mut total_hits: usize = 0;
    for battleship in battleships {
        total_hits += battleship.length();
    }
    if total_hits == hits.len() {
        return true;
    }
    false
}

fn get_guess(axis: &str, max: u8) -> u8 {
    print!("Please type a {} coordinate: ", axis);
    io::stdout().flush().unwrap();
    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to readline");

        let guess: u8 = match guess.trim().parse() {
            Ok(coordinate) => {
                if coordinate > max {
                    // Preparing the print statment
                    print!("Coordinate is out of bounds! Try again: ");
                    // Printing it to the console
                    io::stdout().flush().unwrap();
                    continue;
                }
                coordinate
            },
            Err(_) => {
                print!("Invalid {} coordinate! Try again: ", axis);
                io::stdout().flush().unwrap();
                continue;
            }
        };

        return guess;
    }
}
