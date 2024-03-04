use std::thread::sleep;
use std::time::Duration;
use std::io::{self, Write}; //name

struct AsciiObject {
    ascii: Vec<Vec<char>>, // Each of the rows here need to be the same length 
    //Upper left corner position
    x_pos:usize,
    y_pos:usize,
}

impl AsciiObject {

    
    fn new(ascii_string:String, x_pos: usize, y_pos: usize) -> Self {
        
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

    fn print_object(&mut self) {
        for column in &self.ascii {
            for row in column {
                print!("{}", row);
            }
            println!();
        }
    }

    fn get_width(&mut self) -> usize {
        self.ascii.get(0)
                .expect("Ascii objects should  have at least one element")
                .len()
    }
    fn get_height(&mut self) -> usize {
        self.ascii.len()
    }

    fn update_position(&mut self, x_pos:usize, y_pos:usize) {
        self.x_pos = x_pos;
        self.y_pos = y_pos;
    }
}

struct Frame {
    filler_char: char,
    ascii: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Frame {
    
    fn print_frame(&mut self) {
        for column in &self.ascii {
            for row in column {
                print!("{}", row);
            }
            println!();
        }
    }
    fn new(filler_char: char, width: usize, height: usize) -> Self {
        let ascii = vec![vec![filler_char; width]; height];
        Self { filler_char, ascii, width, height }
    }

    fn add_object(&mut self,  object: &mut AsciiObject) {
        // Replace each line
        for i in 0..object.get_height() {
            // Replace each character in the line
            for j in 0..object.get_width() {
                self.ascii[i + object.y_pos][j +  object.x_pos] = object.ascii[i][j];
            }
        }
    }

    fn remove_object(&mut self,  object: &mut AsciiObject) {
        // Replace each line
        for i in 0..object.get_height() {
            // Replace each character in the line
            for j in 0..object.get_width() {
                self.ascii[i + object.y_pos][j +  object.x_pos] = self.filler_char;
            }
        }
    }
}

fn main() {
    const FRAME_RATE: u64 = 40; // In milliseconds

    let ascii_frog: String  =
        " /\\  (•)___(•)  /\\ \n".to_string() +
        " | \\/         \\/ | \n" +
        "_|  \\         /  |_\n" +
        "       -------      ";

    let ascii_mushroom: String  =
        "   _______   \n".to_string() +
        " /       o \\ \n" +
        "|___  o  ___|\n" +
        "    |___|    \n";
    

    let mut frog = AsciiObject::new(ascii_frog, 10, 10);
    let mut mushroom = AsciiObject::new(ascii_mushroom, 10, 10);
    let mut frame = Frame::new('*', 100, 20);

    

    println!("Name your frog to begin!:");//name
    let mut frog_name = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut frog_name).expect("Failed to read line");

    let mut frog_name_text = AsciiObject::new(frog_name, 0, 0);

    frame.add_object(&mut frog_name_text);

    //sleep(Duration::from_millis(10000));
    
    for i in 0..10 {
        clearscreen::clear().expect("Failed to clear screen!");
        frame.remove_object(&mut frog);
        frog.update_position(10,15 - i);
        frame.add_object(&mut frog);

        frame.remove_object(&mut mushroom);
        mushroom.update_position(80 - i, 14);
        frame.add_object(&mut mushroom);

        frame.print_frame();
        sleep(Duration::from_millis(FRAME_RATE));
    }

    for i in 0..10 {
        clearscreen::clear().expect("Failed to clear screen!");
        frame.remove_object(&mut frog);
        frog.update_position(10,5 + i);
        frame.add_object(&mut frog);

        frame.remove_object(&mut mushroom);
        mushroom.update_position(70 - i, 14);
        frame.add_object(&mut mushroom);

        frame.print_frame();
        sleep(Duration::from_millis(FRAME_RATE));
    }
    
}
