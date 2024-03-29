// Import Structs
use super::ascii_object::AsciiObject; 

pub struct Frame {
    pub filler_char: char,
    pub ascii: Vec<Vec<char>>,
    pub width: isize,
    pub height: isize,
    pub score: isize,
}

impl Frame {

    pub fn new(filler_char: char, width: isize, height: isize, score:isize) -> Self {
        let ascii = vec![vec![filler_char; width as usize]; height as usize];
        Self {
            filler_char,
            ascii,
            width,
            height,
            score
        }
    }


    pub fn print_frame(&mut self) {
        for column in &self.ascii {
            for row in column {
                print!("{}", row);
            }
            println!();
        }
    }

    // // Clears frame by filling it with filler_char
    // pub fn clear_frame(&mut self) {
    //     // Replace each line
    //     for i in 0..self.height as usize {
    //         // Replace each character in the line
    //         for j in 0..self.width as usize {
    //             self.ascii[i][j] = self.filler_char;
    //         }
    //     }
    // }


    // Checks if you can place the object in the frame by seeing if there is any other object already there
    pub fn can_add_object(&mut self, object: &mut AsciiObject) -> bool {

        for i in 0..object.get_height() as usize {
            for j in 0..object.get_width() as usize{
                if self.ascii[i + object.y_pos as usize][j + object.x_pos as usize] != self.filler_char {
                    return false;
                };
            }
        }
        return true;
    }

    // Checks if the object will be in bounds if added to the frame
    pub fn is_object_in_frame_bounds(&mut self, object: &mut AsciiObject) -> bool {
        
        let upper_left_x = object.x_pos;
        let upper_left_y = object.y_pos;

        let lower_right_x = object.x_pos + object.get_width();
        let lower_right_y = object.y_pos + object.get_height();

        return upper_left_x >= 0 && upper_left_y >= 0 && lower_right_x < self.width 
            && lower_right_y < self.height 

    }

    // Adds the AsciiObject (that has a specified location) to the frame
    pub fn add_object(&mut self, object: &mut AsciiObject) {
        // Replace each line
        let height_index:usize = object.get_height() as usize;
        let width_index:usize = object.get_width() as usize;
        for i in 0..height_index {
            // Replace each character in the line
            for j in 0..width_index {
                self.ascii[i + object.y_pos as usize][j + object.x_pos as usize] = object.ascii[i][j];
            }
        }
    }

    // Removes the AsciiObject (that has a specified location) from the frame
    pub fn remove_object(&mut self, object: &mut AsciiObject) {
        // Replace each line
        for i in 0..object.get_height() as usize {
            // Replace each character in the line
            for j in 0..object.get_width() as usize {
                self.ascii[i + object.y_pos as usize][j + object.x_pos as usize] = self.filler_char;
            }
        }
    }
}
