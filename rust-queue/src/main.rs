mod queue;

use queue::queue::{
    Queue
};

fn main() {
    let testQ = Queue::<u32>::new();
    println!("Initialized");
}
