mod queue {
    enum NodeRecord<T> {
        Empty,
        Node { value: T, next: NodeRecord<T> },
    }

    pub struct Queue::<T> {
        head: NodeRecord<T>,
    }

    impl<T> Queue::<T> {
        pub fn new() -> Queue<T> {
            return Queue<T> {
                head: Empty,
            };
        }

        pub fn clear(&self) -> () {
            self.head = Empty;
        }
    }

}