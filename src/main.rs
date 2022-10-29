mod grid;
mod battleship;

use crate::grid::Grid;
use crate::battleship::Battleship;
use crate::battleship::Direction;

fn main() {
    let mut battleships: Vec<Battleship> = Vec::new();

    battleships.push(Battleship::new(0, 0, 2, Direction::Horizontal));
    battleships.push(Battleship::new(3, 0, 3, Direction::Vertical));

    let grid = Grid::build(5,5).unwrap();

    println!("Grid Width: {}, Grid Height {}", grid.get_width(), grid.get_height());

    grid.draw(&battleships);
}
