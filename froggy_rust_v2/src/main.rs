use std::thread::sleep;
use std::time::Duration;
use std::io::{self, Write}; //name

struct Frog {
    frog_standing: Vec<String>,
}

fn main() {
    const FRAME_RATE: u64 = 40; // In milliseconds
    let frog: Frog = Frog {
        frog_standing: vec![
            "".to_string(),
            " /\\  (•)___(•)  /\\".to_string(),
            " | \\/         \\/ |".to_string(),
            "_|  \\         /  |_".to_string(),
            "      -------     ".to_string(),
        ],
    };

    let mut layout: Vec<String> = frog.frog_standing;

    for _ in 0..10 {layout.insert(0, " ".repeat(18));}
    layout.push("_".repeat(30));

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

    for _ in 0..10 {
        clearscreen::clear().expect("Failed to clear screen!");
        
        println!("{}", frog_name.trim().to_string());//name

        for line in &layout {
            println!("{}", line);
        }
        
        layout.remove(0);
        let insert_index = layout.len() - 1;
        layout.insert(insert_index, " ".repeat(18));
        sleep(Duration::from_millis(FRAME_RATE));
    }

    for _ in 0..11 {
        clearscreen::clear().expect("Failed to clear screen!");

        println!("{}", frog_name.trim().to_string());//name

        for line in &layout {
            println!("{}", line);
        }
        
        layout.insert(0, " ".repeat(18));
        let remove_index = layout.len() - 2;
        layout.remove(remove_index);
        sleep(Duration::from_millis(FRAME_RATE));
    }
}
