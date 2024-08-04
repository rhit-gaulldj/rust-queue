mod queue;

use queue::queue::{Queue, QueueContract};

use std::io::{stdin, stdout, Write};

fn main() {
    println!("Initialized");

    let mut input = String::new();
    let mut q1 = Queue::<u32>::new();
    let mut q2 = Queue::<u32>::new();

    loop {
        display_menu();
        get_command(&mut input);
        if input == "Q" || input == "q" {
            return;
        }

        dispatch_command(&input, &mut q1, &mut q2);
    }
}

fn display_menu() -> () {
    println!("\tCommand - Result");
    println!("\t\te - enqueue");
    println!("\t\td - dequeue");
    println!("\t\tr - replaceFront");
    println!("\t\tf - front");
    println!("\t\tz - length");
    println!("\t\tx - transferFrom");
    println!("\t\t= - copy");
    println!("\t\tp - print/display");
    println!("\t\tc - clear");
    println!("\t\tq - quit");

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

fn do_enqueue(q1: &mut Queue<u32>, q2: &mut Queue<u32>) -> () {
    let index = get_index();
    println!("Not implemented");
}

fn do_dequeue(q1: &mut Queue<u32>, q2: &mut Queue<u32>) -> () {
    let index = get_index();
    println!("Not implemented");
}

fn do_replace_front(q1: &mut Queue<u32>, q2: &mut Queue<u32>) -> () {
    let index = get_index();
    println!("Not implemented");
}

fn do_front(q1: &mut Queue<u32>, q2: &mut Queue<u32>) -> () {
    let index = get_index();
    if index == 1 {
        if q1.length() > 0 {
            println!("Front of q{}: {}", index, q1.front());
        } else {
            println!("Error! Cannot get front of an empty queue");
        }
    } else {
        if q2.length() > 0 {
            println!("Front of q{}: {}", index, q2.front());
        } else {
            println!("Error! Cannot get front of an empty queue");
        }
    }
}

fn do_length(q1: &mut Queue<u32>, q2: &mut Queue<u32>) -> () {
    let index = get_index();
    if index == 1 {
        println!("Length of q{}: {}", index, q1.length());
    } else {
        println!("Length of q{}: {}", index, q2.length());
    }
}

fn do_transfer_from(q1: &mut Queue<u32>, q2: &mut Queue<u32>) -> () {
    println!("Not implemented");
}

fn do_copy(q1: &mut Queue<u32>, q2: &mut Queue<u32>) -> () {
    println!("Not implemented");
}

fn do_display(q1: &mut Queue<u32>, q2: &mut Queue<u32>) -> () {
    let index = get_index();
    println!("Not implemented");
}

fn do_clear(q1: &mut Queue<u32>, q2: &mut Queue<u32>) -> () {
    let index = get_index();
    if index == 1 {
        q1.clear();
    } else {
        q2.clear();
    }
    println!("Cleared q{}", index);
}

fn dispatch_command(cmd: &String, q1: &mut Queue<u32>, q2: &mut Queue<u32>) -> () {
    match cmd.as_str() {
        "e" => do_enqueue(q1, q2),
        "d" => do_dequeue(q1, q2),
        "r" => do_replace_front(q1, q2),
        "f" => do_front(q1, q2),
        "z" => do_length(q1, q2),
        "x" => do_transfer_from(q1, q2),
        "=" => do_copy(q1, q2),
        "p" => do_display(q1, q2),
        "c" => do_clear(q1, q2),
        "q" => println!("Exiting command interpreter"),
        _ => println!("Invalid Command!"),
    }
}
