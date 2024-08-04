pub mod queue {
    pub trait QueueContract<T> {
        fn clear(&mut self) -> ();
        // fn transferFrom(&mut self, other: &mut Queue::<T>) -> ();
        // fn enqueue(&mut self, x: T) -> ();
        // fn dequeue(&mut self) -> T;
        // fn replaceFront(&mut self, x: T) -> T;
        fn front(&self) -> &T;
        fn length(&self) -> u32;
        // Note: Cannot overload the Rust assignment operator
        // fn assign(&mut self, other: &Queue::<T>) -> ();
    }

    enum NodeRecord<T> {
        Empty,
        Node { value: T, next: Box<NodeRecord<T>> },
    }

    pub struct Queue<T> {
        head: NodeRecord<T>,
        length: u32,
    }

    impl<T> QueueContract<T> for Queue<T> {
        fn clear(&mut self) -> () {
            self.head = NodeRecord::Empty;
        }

        fn front(&self) -> &T {
            // Unlike in 373, must define every possible path in the match expression
            // This means we don't have undefined behavior here, but we will terminate
            // if the "external contract" of this function is violated

            match &(self.head) {
                NodeRecord::Empty => panic!("Cannot call 'front' on an empty queue."),
                NodeRecord::Node { value, next: _ } => {
                    value
                }
            }
        }

        fn length(&self) -> u32 {
            self.length
        }

        // fn enqueue(&mut self, x: T) -> () {
        //     let newNode: NodeRecord<T> = NodeRecord::Node {
        //         value: x,
        //         next: Box::new(NodeRecord::Empty),
        //     };

        //     match &self.head {
        //         NodeRecord::Empty => self.head = newNode,
        //         NodeRecord::Node { value, next } => {
        //             let current: NodeRecord<T> = self.head;
        //             loop {
        //                 match current.next {
        //                     NodeRecord::Empty => {
        //                         current.next = newNode;
        //                         break;
        //                     },
        //                     NodeRecord::Node { value, next } => {
        //                         current = next;
        //                     },
        //                 };
        //             }
        //         },
        //     };
        // }
    }

    impl<T> Queue<T> {
        pub fn new() -> Queue<T> {
            return Queue {
                head: NodeRecord::Empty,
                length: 0,
            };
        }
    }

}