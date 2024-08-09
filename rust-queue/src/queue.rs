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

    type NodePointer<T> = Option<Box<Node<T>>>;

    struct Node<T> {
        item: T,
        next: NodePointer<T>,
    }

    pub struct Queue<T> {
        head: NodePointer<T>,
        length: u32,
    }

    impl<T> QueueContract<T> for Queue<T> {
        fn clear(&mut self) -> () {
            self.head = None;
            self.length = 0;
        }

        fn front(&self) -> &T {
            // Must define every path, cannot have undefined behavior like C++
            // So we have to explicitly handle the null case (which is Option None)
            match self.head.as_ref() {
                None => panic!("Cannot call front on an empty queue!"),
                Some(node) => &node.item,
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
        //             let prev = NodeRecord::<T>::Empty;
        //             let mut current = &mut self.head;
        //             loop {
        //                 match current {
        //                     NodeRecord::Empty => {
                                
        //                         break;
        //                     },
        //                     NodeRecord::Node { value, next } => {
        //                         current = *next;
        //                     },
        //                 };
        //             }
        //         },
        //     };
        // }
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