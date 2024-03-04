// Import Structs
use super::ascii_object::AsciiObject; 

pub struct Frame {
    pub filler_char: char,
    pub ascii: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl Frame {
    pub fn print_frame(&mut self) {
        for column in &self.ascii {
            for row in column {
                print!("{}", row);
            }
            println!();
        }
    }
    pub fn new(filler_char: char, width: usize, height: usize) -> Self {
        let ascii = vec![vec![filler_char; width]; height];
        Self {
            filler_char,
            ascii,
            width,
            height,
        }
    }

    pub fn add_object(&mut self, object: &mut AsciiObject) {
        // Replace each line
        for i in 0..object.get_height() {
            // Replace each character in the line
            for j in 0..object.get_width() {
                self.ascii[i + object.y_pos][j + object.x_pos] = object.ascii[i][j];
            }
        }
    }

    pub fn remove_object(&mut self, object: &mut AsciiObject) {
        // Replace each line
        for i in 0..object.get_height() {
            // Replace each character in the line
            for j in 0..object.get_width() {
                self.ascii[i + object.y_pos][j + object.x_pos] = self.filler_char;
            }
        }
    }
}
