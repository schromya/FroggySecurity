use std::thread::sleep;
use std::time::Duration;
use std::io::{self, Write}; //name
use crossterm::{
    event::{self, poll, read, Event, KeyCode},
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

        if frog_name.trim().chars().all(|c| c.is_alphabetic()) {
            println!("Frog name is valid: {}", frog_name.trim());
            break;
        } else {
            println!("Error! Frog name must contain only letters.");
        }
    }

    return frog_name;
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
    // if !frame.can_add_object(object) {
    //     println!("Game Over!");
    //     std::process::exit(0); // TODO better version
    // }

    frame.add_object(object); // Re-add object

}

fn move_object_up(object: &mut AsciiObject, frame: &mut Frame)  {

    frame.remove_object(object); // Remove object from frame

    object.y_pos -= 1; // Move object position up

    // Update positon to bottom of screen if out of bounds on the top
    if !frame.is_object_in_frame_bounds(object) {
        object.y_pos = frame.height - object.get_height() - 1;
    }

    // Check if interfering objects
    // if !frame.can_add_object(object) {
    //     println!("Game Over!");
    //     std::process::exit(0); // TODO better version
    // }

    frame.add_object(object); // Re-add object

}

// Move object up and down oscillating
// Returns bool of if landed on the ground  (true if landed on ground)
fn move_jump(object: &mut AsciiObject, frame: &mut Frame)  -> bool {

    let mut landed:bool = false;

    frame.remove_object(object); // Remove object from frame


    if object.movement_direction == "up" {
        object.y_pos -= 1; // Move object position up
    } else { // Else down
        object.y_pos += 1; // Move object position down
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
            object.y_pos  = frame.height - object.get_height() - 2;
        }
    }

    // Check if interfering objects
    // if !frame.can_add_object(object) {
    //     println!("Game Over!");
    //     std::process::exit(0); // TODO better version
    // }

    frame.add_object(object); // Re-add object
    return landed;

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
    

    let mut frog = AsciiObject::new(ascii_frog, 10, 10, "up".to_string());
    let mut mushroom = AsciiObject::new(ascii_mushroom, 10, 14, "left".to_string());  
    let mut frog_name_text = AsciiObject::new(get_frog_name(), 5, 1, "none".to_string());

    let mut frame = Frame::new('*', 100, 20);  

    frame.add_object(&mut frog_name_text);

    //sleep(Duration::from_millis(10000));

    let mut landed:bool = false;
    loop {
        clearscreen::clear().expect("Failed to clear screen!");
        move_object_left(&mut mushroom, &mut frame);
        frame.add_object(&mut frog_name_text);

        // Jump if space is pressed or coninue jumping if in mid-jump

        if !landed  ||  is_space_input(){
            landed = move_jump(&mut frog, &mut frame);
        } else {
            frame.add_object(&mut frog);
        }

        frame.print_frame();

        sleep(Duration::from_millis(FRAME_RATE));

    }

}
