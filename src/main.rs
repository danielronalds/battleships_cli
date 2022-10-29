mod grid;

use crate::grid::Grid;

fn main() {
    let grid = Grid::build(5,5).unwrap();

    println!("Grid Width: {}, Grid Height {}", grid.get_width(), grid.get_height());

    grid.draw();
}
