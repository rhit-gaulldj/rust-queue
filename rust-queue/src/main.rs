mod queue;

use queue::queue::Queue;

use std::io::{stdin, stdout, Write};

fn main() {
    println!("Initialized");

    let mut input = String::new();

    loop {
        display_menu();
        get_command(&mut input);
        if input == "Q" || input == "q" {
            return;
        }

        // TODO: Dispatch command
    }
}

fn display_menu() -> () {
    println!("\tCommand - Result");
    println!("\t\tg - toggle display menu");
    println!("\t\te - enqueue");
    println!("\t\td - dequeue");
    println!("\t\tr - replaceFront");
    println!("\t\tf - front");
    println!("\t\tz - length");
    println!("\t\tx - transferFrom");
    println!("\t\t= - copy");

    print!("command: ");
}

// https://users.rust-lang.org/t/how-to-get-user-input/5176/3
fn get_command(s: &mut String) -> () {
    let _ = stdout().flush();
    stdin().read_line(s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
}

// https://stackoverflow.com/questions/30355185/how-to-read-an-integer-input-from-the-user-in-rust-1-0
fn get_index() -> u32 {
    let mut input_text = String::new();

    loop {
        print!("Which index? 1 or 2: ");
        stdin().read_line(&mut input_text).expect("Did not enter a correct string");

        let trimmed = input_text.trim();
        match trimmed.parse::<u32>() {
            Ok(i) => {
                if i == 1 || i == 2 {
                    return i;
                }
                println!("Please enter 1 or 2!");
            },
            Err(..) => println!("Please enter a number!")
        }
    }
}
