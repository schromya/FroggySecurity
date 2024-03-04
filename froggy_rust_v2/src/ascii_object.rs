pub struct AsciiObject {
    pub ascii: Vec<Vec<char>>, // Each of the rows here need to be the same length 
    //Upper left corner position
    pub x_pos:usize,
    pub y_pos:usize,
}

impl AsciiObject {

    
    pub fn new(ascii_string:String, x_pos: usize, y_pos: usize) -> Self {
        
        // Converts ascii string to vector of vector of chars
        let mut ascii: Vec<Vec<char>> = Vec::new();
        let mut line = Vec::new();

        for char in ascii_string.chars() {
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

        Self { ascii, x_pos, y_pos }
    }

    pub fn print_object(&mut self) {
        for column in &self.ascii {
            for row in column {
                print!("{}", row);
            }
            println!();
        }
    }

    pub fn get_width(&mut self) -> usize {
        self.ascii.get(0)
                .expect("Ascii objects should  have at least one element")
                .len()
    }
    pub fn get_height(&mut self) -> usize {
        self.ascii.len()
    }

    pub fn update_position(&mut self, x_pos:usize, y_pos:usize) {
        self.x_pos = x_pos;
        self.y_pos = y_pos;
    }
}