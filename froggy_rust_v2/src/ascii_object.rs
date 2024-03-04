pub struct AsciiObject {
    pub ascii: Vec<Vec<char>>, // Each of the rows here need to be the same length 
    //Upper left corner position
    pub x_pos:isize,
    pub y_pos:isize,
    pub movement_direction:String,  // TODO make this enum?
}

impl AsciiObject {


    pub fn new(ascii_string:String, x_pos: isize, y_pos: isize, movement_direction:String) -> Self {
        
        // Converts ascii string to vector of vector of chars
        let ascii: Vec<Vec<char>> = Self::convert_str_to_vector(ascii_string);
        Self { ascii, x_pos, y_pos, movement_direction}
    }

    // Converts string to vector of vectors (\n are new rows in the matrix)
    // This is a static function (accessed like AsciiObject::convert_str_to_vector(...) )
    pub fn convert_str_to_vector(string:String) ->  Vec<Vec<char>>{
        let mut ascii: Vec<Vec<char>> = Vec::new();
        let mut line = Vec::new();

        for char in string.chars() {
            if char == '\n' {
                ascii.push(line);
                line = Vec::new();
            } else {
                line.push(char);
            }
        }
        if !line.is_empty() {
            // Push last line
            ascii.push(line);
        }

        return ascii

    }


    // pub fn print_object(&mut self) {
    //     for column in &self.ascii {
    //         for row in column {
    //             print!("{}", row);
    //         }
    //         println!();
    //     }
    // }

    pub fn get_width(&mut self) -> isize {
        self.ascii.get(0)
                .expect("Ascii objects should  have at least one element")
                .len() as isize
    }
    pub fn get_height(&mut self) -> isize {
        self.ascii.len() as isize
    }
}