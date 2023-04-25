pub struct Queue<T> {
    queue: Vec<Option<T>>,
    head: usize,
    tail: usize,
}

impl<T> Queue<T> {
    pub fn new(capacity: usize) -> Self {
        Queue {
            queue: Vec::with_capacity(capacity),
            head: 0,
            tail: 0,
        }
    }

    pub fn is_full(&self) -> bool {
        self.queue.capacity() < self.queue.len()
    }

    pub fn enqueue(&mut self, item: T) -> bool {
        if self.is_full() {
            return false;
        }

        self.queue.push(Some(item));
        self.tail += 1;
        if self.tail + 1 < self.queue.capacity() {
            self.tail += 1;
        } else {
            self.tail = 0;
        }
        true
    }

    pub fn dequeue(&mut self) -> Option<T> {
        let item = self.queue[self.head].take();
        println!("{} {}", self.head, self.queue.capacity());
        if self.head + 1 < self.queue.capacity() {
            self.head += 1;
        } else {
            self.head = 0;
        }
        item
    }

    pub fn size(&self) -> usize {
        self.queue.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue() {
        let mut queue = Queue::new(2);
        assert!(queue.enqueue("a"));
        assert!(queue.enqueue("b"));
        assert_eq!(queue.size(), 2);
        assert_eq!(queue.dequeue().unwrap(), "a");
        assert_eq!(queue.dequeue().unwrap(), "b");
    }

    #[test]
    fn test_dequeue() {
        let mut queue = Queue::new(2);
        queue.enqueue(0);
        queue.enqueue(1);
        assert_eq!(queue.size(), 2);
        assert_eq!(queue.dequeue().unwrap(), 0);
        assert_eq!(queue.dequeue().unwrap(), 1);
        assert!(queue.dequeue().is_none());
    }
}
