use std::thread::sleep;
use std::time::Duration;

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

    for _ in 0..10 {
        clearscreen::clear().expect("Failed to clear screen!");
        
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
        for line in &layout {
            println!("{}", line);
        }
        
        layout.insert(0, " ".repeat(18));
        let remove_index = layout.len() - 2;
        layout.remove(remove_index);
        sleep(Duration::from_millis(FRAME_RATE));
    }
}
