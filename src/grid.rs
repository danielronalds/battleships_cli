use crate::Battleship;

// Struct to store a grid
pub struct Grid {
    width: u8,
    height: u8
}

impl Grid {
    pub fn build(width: u8, height: u8) -> Result<Grid, &'static str> {
        // Returning errors if either the width or height are less than 1
        if width < 1 {
            return Err("The width of the grid cannot be less than 1!")
        }
        if height < 1 {
            return Err("The height of the grid cannot be less than 1!")
        }

        // Returning a successful result
        Ok(Grid {width, height})
    }

    pub fn width(&self) -> &u8 {
        &self.width
    }


    pub fn height(&self) -> &u8 {
        &self.height
    }

    pub fn draw(&self, battleships: &Vec<Battleship>) {
        // Drawing the top header
        self.draw_header();

        for y in 0..self.height {
            let mut row = String::from("|");

            for x in 0..self.width {
                let mut border_drawn = false;

                for battleship in battleships {
                    if battleship.occupies(x, y) {
                        row.push_str(" B |");
                        border_drawn = true;
                        break;
                    }
                }

                // Drawing the border if it wasn't already drawn
                if !border_drawn { row.push_str("   |"); } 
            }

            println!("{}", row);
            self.draw_header();
        }
    }

    fn draw_header(&self) {
        let mut header = String::from("+");

        for _i in 0..self.width {
            header.push_str("---+");
        }

        println!("{}", header);
    }
}
