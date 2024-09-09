pub mod queue {
    use std::ptr::NonNull;

    pub trait QueueContract<T> where T : core::fmt::Display {
        fn clear(&mut self) -> ();
        // fn transferFrom(&mut self, other: &mut Queue::<T>) -> ();
        fn enqueue(&mut self, x: T) -> ();
        fn dequeue(&mut self) -> T;
        fn replace_front(&mut self, x: T) -> T;
        fn front(&self) -> &T;
        fn length(&self) -> u32;
        // Note: Cannot overload the Rust assignment operator
        // fn assign(&mut self, other: &Queue::<T>) -> ();
        fn print(&self) -> ();
    }

    type NodePointer<T> = Option<NonNull<Node<T>>>;

    struct Node<T> {
        item: T,
        next: NodePointer<T>,
    }

    pub struct Queue<T> {
        head: NodePointer<T>,
        length: u32,
    }

    impl<T> QueueContract<T> for Queue<T> where T : core::fmt::Display {
        fn clear(&mut self) -> () {
            // Rust should clean up for us
            self.head = None;
            self.length = 0;
        }

        fn front(&self) -> &T {
            // Must define every path, cannot have undefined behavior like C++
            // So we have to explicitly handle the null case (which is Option None)
            match self.head.as_ref() {
                None => panic!("Cannot call front on an empty queue!"),
                Some(node) => {
                    unsafe {
                        &(*node.as_ptr()).item
                    }
                },
            }
        }

        fn replace_front(&mut self, x: T) -> T {
            match self.head.as_ref() {
                None => panic!("Cannot call replaceFront on an empty queue!"),
                Some(node) => {
                    let ptr: *mut Node<T> = node.as_ptr();
                    unsafe {
                        let boxed_node = Box::from_raw(ptr);
                        let item = (*boxed_node).item;
                        (*ptr).item = x;
                        item
                    }
                },
            }
        }

        fn length(&self) -> u32 {
            self.length
        }

        fn enqueue(&mut self, item: T) -> () {
            unsafe {
                let new_node: NodePointer<T> = Some(NonNull::new_unchecked(
                    Box::into_raw(Box::new(
                        Node {
                            item: item,
                            next: None,
                        }
                    ))
                ));

                self.length += 1;

                // A bit messy... check if head is null first. If so, replace it
                // These variables are only used when head is not null
                let mut current_node: NonNull<Node<T>>;
                match &mut self.head {
                    None => {
                        self.head = new_node;
                        return;
                    },
                    Some(boxed) => {
                        current_node = *boxed;
                    },
                };

                loop {
                    match (*current_node.as_ptr()).next {
                        None => {
                            (*current_node.as_ptr()).next = new_node;
                            return;
                        },
                        Some(next) => {
                            current_node = next;
                        }
                    }
                }
            }
        }

        fn dequeue(&mut self) -> T {
            self.length -= 1;
            match &mut self.head {
                None => panic!("Cannot call dequeue on an empty queue!"),
                Some(ptr) => {
                    // Move the head to the next, and return the value at the head
                    unsafe {
                        let node = ptr.as_ptr();
                        // Box stuff magically makes this compile?
                        let boxed_node = Box::from_raw(node);
                        let item = (*boxed_node).item;
                        self.head = (*node).next;
                        item
                    }
                },
            }
        }

        fn print(&self) -> () {
            let mut current_node: &NodePointer<T> = &self.head;
            print!("[ ");
            let mut is_first: bool = true;
            loop {
                match current_node {
                    None => break,
                    Some(pointer) => {
                        if !is_first {
                            print!(", ");
                        } else {
                            is_first = false;
                        }

                        // Keep unsafe blocks small
                        unsafe {
                            print!("{}", (*pointer.as_ptr()).item);
                            
                            current_node = &(*pointer.as_ptr()).next;
                        }
                    },
                }
            }

            print!(" ]");
        }

    }

    impl<T> Queue<T> {
        pub fn new() -> Self {
            return Queue {
                head: None,
                length: 0,
            };
        }
    }

}