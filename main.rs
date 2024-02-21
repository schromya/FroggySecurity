use std::io;

struct Frog;

impl Frog {
    fn new() -> Self {
        println!("Welcome to Frog World!");
        Frog
    }

    fn meet(&self) {
        println!("Meeting Froggy");
    }

    fn feed(&self) {
        println!("Feeding Froggy");
    }

    fn chat(&self) {
        println!("Chatting with Froggy");
    }
}

fn main() {
    if check_password() {
        let my_frog = Frog::new();

        my_frog.meet();
        my_frog.feed();
        my_frog.chat();
    } else {
        println!("Incorrect password! Access denied.");
    }
}

fn check_password() -> bool {
    let correct_password = "network";
    println!("Enter password to access Frog World: ");
    let mut input = String::new();

    if io::stdin().read_line(&mut input).is_ok() {
        input.trim() == correct_password
    } else {
        false
    }
}

