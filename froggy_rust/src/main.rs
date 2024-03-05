use std::thread::sleep;
use std::time::Duration;
use std::io::{self, Write}; //name
use crossterm::{
    event::{ poll, read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

// Import Structs
mod ascii_object;  
use ascii_object::AsciiObject; 
mod frame;  
use frame::Frame; 


fn get_frog_name() -> String{
    let mut frog_name;//name
    loop {
        println!("Name your frog to begin! (Letters only):");
        frog_name = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut frog_name).expect("Failed to read line");

        if frog_name.trim().chars().all(|c| c.is_alphabetic()) && frog_name.trim().len() <= 5 {
            println!("Frog name is valid: {}", frog_name.trim());
            break;
        } else {
            println!("Error! Frog name must contain only letters and be less than 5 characters");
        }
    }

    return frog_name.trim().to_string();
}

// Check if space is pressed as an input
fn is_space_input() -> bool {

    // Need to use crossterm library bc io stdin isn't asynchronous
    // Require raw mode to allow reading single key presses
    enable_raw_mode().expect("Failed to enable raw mode");

    let mut space_pressed = false;

    // Check if there's an event within 0 milliseconds (non-blocking)
    if poll(Duration::from_millis(0)).expect("Failed to poll for event") {
        // If there's an event, check if it's a key press
        if let Ok(Event::Key(key_event)) = read() {
            space_pressed = key_event.code == KeyCode::Char(' ');
        }
    }

    // Disable raw mode to restore terminal behavior
    disable_raw_mode().expect("Failed to disable raw mode");

    return space_pressed;


}

fn move_object_left(object: &mut AsciiObject, frame: &mut Frame)  {

    frame.remove_object(object); // Remove object from frame

    object.x_pos -= 1; // Move object position left

    // Update positon to right of screen if out of bounds on the left
    if !frame.is_object_in_frame_bounds(object) {
        object.x_pos = frame.width - object.get_width() - 1;
    }

    // Check if interfering objects
    if !frame.can_add_object(object) {
        game_over();
    }

    frame.add_object(object); // Re-add object

}


// Move object up and down oscillating
// Returns bool of if landed on the ground  (true if landed on ground)
fn move_jump(object: &mut AsciiObject, frame: &mut Frame,ascii_jumping_frog: &String, ascii_frog: &String)  -> bool {

    let mut landed:bool = false;

    frame.remove_object(object); // Remove object from frameF


    if object.movement_direction == "up" {
        object.y_pos -= 1; // Move object position up
        object.ascii = AsciiObject::convert_str_to_vector(ascii_jumping_frog.clone());
    } else { // Else down
        object.y_pos += 1; // Move object position down
        if object.y_pos  == frame.height - object.get_height() - 1 { // Check if object landed
            object.ascii = AsciiObject::convert_str_to_vector(ascii_frog.clone());
        }
    }

    // Update positon to bottom of screen if out of bounds on the top
    if !frame.is_object_in_frame_bounds(object) {

        // Start moving the object down if currently at top
        if object.movement_direction == "up" {
            object.movement_direction = "down".to_string();
            object.y_pos = 1;
        // Else land object if on ground
        } else { // Else down
            landed = true;
            object.movement_direction = "up".to_string();
            object.y_pos  = frame.height - object.get_height() - 1;
        }
    }

    // Check if interfering objects
    if !frame.can_add_object(object) {
        game_over();
    }

    frame.add_object(object); // Re-add object
    return landed;

}

fn game_over() {
    let ascii_end_game: String  =
    " _______________ \n".to_string() +
    "|               |\n" +
    "|   GAME OVER   |\n" +
    "|_______________|\n";

    let mut frame = Frame::new('.', 100, 20);  

    let mut end_game_text = AsciiObject::new(ascii_end_game, 41, 8, "none".to_string());
    frame.add_object(&mut end_game_text);

    clearscreen::clear().expect("Failed to clear screen!");
    frame.print_frame();

    std::process::exit(0); // TODO better version
}

fn game_win() {
    let ascii_end_game: String  =
    " _____________________________ \n".to_string() +
    "|                             |\n" +
    "|   YOU WON (>= 1000 points)  |\n" +
    "|_____________________________|\n";

    let mut frame = Frame::new('.', 100, 20);  

    let mut end_game_text = AsciiObject::new(ascii_end_game, 35, 8, "none".to_string());
    frame.add_object(&mut end_game_text);

    clearscreen::clear().expect("Failed to clear screen!");
    frame.print_frame();

    std::process::exit(0); // TODO better version
}

fn main() {
    const FRAME_RATE: u64 = 15; // In milliseconds
    let ascii_frog: String  =
        "                   \n".to_string() +
        "                   \n" +
        " /\\  (•)___(•)  /\\ \n" +
        " | \\/         \\/ | \n" +
        "_|  \\         /  |_\n" +
        "      -------      ";

    // let ascii_jumping_frog: String  =
    //     "  (•)___(•)  \n".to_string() +
    //     " /         \\ \n" +
    //     "|\\         /\\ \n" +
    //     "|  -------  | \n";      
          
    let ascii_jumping_frog: String  =
    "      (•)___(•)    \n".to_string() +
    "     /         \\   \n" +
    "    |\\         /|  \n" +
    "    |  -------  |  \n" +
    "    |           |  \n" +
    "    /           \\  ";

    // "   \\ (•)___(•) /   \n".to_string() +
    // "     \\       /     \n" +
    // "      |  |  |      \n" +
    // "      |  |  |      \n";

    let ascii_mushroom: String  =
        "  _____  \n".to_string() +
        " /   o \\ \n" +
        "|__o  __|\n" +
        "   |_|   \n";
    
    let ascii_name: String = "| ".to_string() + &get_frog_name() + " |";
    let ascii_name_len: usize = ascii_name.len();
    let ascii_name: String = "-".repeat(ascii_name_len) + "\n" + &ascii_name + "\n" + &"-".repeat(ascii_name_len);

    let mut frame = Frame::new(' ', 100, 20);  

    let mut frog = AsciiObject::new(ascii_frog.clone(), 20, frame.height - 7, "up".to_string());
    let mut mushroom = AsciiObject::new(ascii_mushroom, frame.width - 10, frame.height - 5, "left".to_string());  
    let mut frog_name_text = AsciiObject::new(ascii_name, 5, 1, "none".to_string());
    let mut points_text = AsciiObject::new("Points: 0".to_string(), frame.width - 15, 2, "none".to_string());

    sleep(Duration::from_millis(1000));
    clearscreen::clear().expect("Failed to clear screen!");
    println!("Game beginning in 3...");
    sleep(Duration::from_millis(1000));
    clearscreen::clear().expect("Failed to clear screen!");
    println!("Game beginning in 3... 2...");
    sleep(Duration::from_millis(1000));
    clearscreen::clear().expect("Failed to clear screen!");
    println!("Game beginning in 3... 2... 1... ");
    sleep(Duration::from_millis(1000));


    let mut landed:bool = true;
    let mut is_frog_jumping = false;

    let mut frog_frame_rate_count: i32 = 0;
    let mut points_rate_count: i32 = 0;
    let mut points: i32 = 0;
    loop {
        clearscreen::clear().expect("Failed to clear screen!");
        
        frame.add_object(&mut frog_name_text);
        frame.add_object(&mut points_text);
        

        move_object_left(&mut mushroom, &mut frame);

        // Jump if space is pressed or continue jumping if in mid-jump
        // Update the frog every 2 iterations
        if (frog_frame_rate_count == 2 && !landed)  ||  is_space_input(){
            if !is_frog_jumping { // Start of jump
                is_frog_jumping = true;
                frog.ascii = AsciiObject::convert_str_to_vector(ascii_jumping_frog.clone());
            }
            landed = move_jump(&mut frog, &mut frame, &ascii_jumping_frog, &ascii_frog);
            if landed { // End of jump
                is_frog_jumping = false;
                frog.ascii = AsciiObject::convert_str_to_vector(ascii_frog.clone());
            }
            frog_frame_rate_count = 0
        } else {
            frame.add_object(&mut frog);
            
        }

        // Update the score every 8 iterations
        if points_rate_count == 8 {
            points += 1;
            points_rate_count = 0;
        }
        if points >= 1000 {
            game_win();
        }

        frog_frame_rate_count += 1;
        points_rate_count += 1;

        // Update points
        points_text.ascii = AsciiObject::convert_str_to_vector(
            " Points: ".to_string() + &points.to_string()); 

        frame.print_frame();

        sleep(Duration::from_millis(FRAME_RATE));

    }

}
