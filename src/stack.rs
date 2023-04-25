pub struct Stack<T> {
    items: Vec<T>,
    size: i32,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            items: Vec::new(),
            size: 0,
        }
    }

    pub fn put(&mut self, item: T) {
        self.items.push(item);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            self.size -= 1;
            self.items.pop()
        }
    }

    pub fn top(&mut self) -> Option<&T> {
        self.items.last()
    }

    pub fn size(&self) -> i32 {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put() {
        let mut stack = Stack::new();
        stack.put(1);
        stack.put(2);
        assert_eq!(stack.top().unwrap().clone(), 2);
        assert_eq!(stack.size(), 2);
    }

    #[test]
    fn test_pop() {
        let mut stack = Stack::new();
        // Pop from empty stack
        assert!(stack.pop().is_none());
        stack.put(0);
        stack.put(1);
        assert_eq!(stack.size(), 2);
        assert_eq!(stack.pop().unwrap(), 1);
        assert_eq!(stack.pop().unwrap(), 0);
        assert!(stack.pop().is_none());
    }

    #[test]
    fn test_top() {
        let mut stack = Stack::new();

        assert!(stack.top().is_none());
        stack.put(0);
        stack.put(1);
        assert_eq!(stack.top().unwrap().clone(), 1);
        assert_eq!(stack.top().unwrap().clone(), 1);
        assert_eq!(stack.size(), 2);
    }
}
