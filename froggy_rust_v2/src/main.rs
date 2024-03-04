use std::thread::sleep;
use std::time::Duration;
use std::io::{self, Write}; //name

// Import Structs
mod ascii_object;  
use ascii_object::AsciiObject; 
mod frame;  
use frame::Frame; 


fn get_frog_name() -> String{
    let mut frog_name;//name
    loop {
        println!("Name your frog to begin!(Letters only):");
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
    let mut frog_name_text = AsciiObject::new(get_frog_name(), 0, 0);

    let mut frame = Frame::new('*', 100, 20);  

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
