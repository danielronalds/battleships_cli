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

    pub fn draw(&self) {
        draw_header();
    }

    fn draw_header(&self) {
        let mut header = "+";

        for i in 0..self.width {
            header += "---+";
        }

        println!("{}", header);
    }
}
