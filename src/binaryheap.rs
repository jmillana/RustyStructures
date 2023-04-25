//! A simple implementation of a binary heap
//!
//! The implemented binary heap can be use as both MinHeap and MaxHeap
enum HeapDirection {
    MAX,
    MIN,
}

pub struct BinaryHeap<T> {
    data: Vec<T>,
    _mode: HeapDirection,
}

impl<T: Ord> BinaryHeap<T> {
    /// Create an empty `BinaryHeap` as max-heap.
    pub fn new() -> Self {
        BinaryHeap {
            data: Vec::new(),
            _mode: HeapDirection::MAX,
        }
    }

    /// Create an empty `BinaryHeap` as min-heap.
    pub fn min_heap() -> Self {
        BinaryHeap {
            data: Vec::new(),
            _mode: HeapDirection::MIN,
        }
    }

    /// Create an empty `BinaryHeap` as max-heap.
    pub fn max_heap() -> Self {
        BinaryHeap {
            data: Vec::new(),
            _mode: HeapDirection::MAX,
        }
    }

    /// Returns the index to the parent of the given node index.
    pub fn parent(i: usize) -> usize {
        return (i - 1) / 2;
    }

    /// Returns the left child index of the given node index.
    fn left(idx: usize) -> usize {
        return 2 * idx + 1;
    }

    /// Returns the right child index of the given node index.
    fn right(idx: usize) -> usize {
        return 2 * idx + 2;
    }

    /// Insert a item into the heap.
    pub fn push(&mut self, item: T) {
        self.data.push(item);
        let mut i = self.data.len() - 1;
        while i != 0 && self.check_order(i, Self::parent(i)) {
            let p = Self::parent(i);
            self.data.swap(i, p);
            i = p;
        }
    }

    /// Remove the greatest/smallest item from the binary heap
    /// depending on the selected heap, max-heap or min-heap, or `None` if empty.
    ///
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        if self.data.len() == 1 {
            return self.data.pop();
        }
        // Swap the first element with the last
        let size = self.data.len() - 1;
        self.data.swap(0, size);
        let root = self.data.pop();
        self.heapify(0);
        return root;
    }

    /// Removes the item at the given index.
    pub fn delete(&mut self, idx: usize) -> Option<T> {
        if self.is_empty() || idx >= self.len() {
            return None;
        }
        self.data.swap(0, idx);
        return self.pop();
    }

    /// Check's and updates the subtree at a given index.
    pub fn heapify(&mut self, idx: usize) {
        let len = self.data.len();
        if len == 1 {
            return;
        }
        let left = Self::left(idx);
        let right = Self::right(idx);
        let mut edge_idx = idx;
        if left < len && self.check_order(left, idx) {
            edge_idx = left;
        }
        if right < len && self.check_order(right, edge_idx) {
            edge_idx = right;
        }
        if edge_idx != idx {
            self.data.swap(idx, edge_idx);
            self.heapify(edge_idx);
        }
    }

    /// Checks the order of the given index depending on the kind of heap.
    #[inline(always)]
    pub fn check_order(&self, left_idx: usize, right_idx: usize) -> bool {
        match self._mode {
            HeapDirection::MAX => return self.data[left_idx] > self.data[right_idx],
            HeapDirection::MIN => return self.data[left_idx] < self.data[right_idx],
        }
    }

    /// Returns a reference to the top element of the heap.
    pub fn peek(&self) -> Option<&T> {
        return self.data.get(0);
    }

    pub fn len(&self) -> usize {
        return self.data.len();
    }

    pub fn is_empty(&self) -> bool {
        return self.len() == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_to_max_heap() {
        let mut max_heap = BinaryHeap::new();
        max_heap.push(3);
        max_heap.push(2);
        max_heap.push(5);
        max_heap.push(2);
        assert_eq!(max_heap.peek(), Some(&5));
        assert_eq!(max_heap.len(), 4);
    }

    #[test]
    fn test_push_to_min_heap() {
        let mut min_heap = BinaryHeap::min_heap();
        min_heap.push(3);
        min_heap.push(2);
        min_heap.push(1);
        min_heap.push(7);
        assert_eq!(min_heap.peek(), Some(&1));
        assert_eq!(min_heap.len(), 4);
    }

    #[test]
    fn test_remove_items() {
        let mut min_heap = BinaryHeap::min_heap();
        min_heap.push(7);
        min_heap.push(4);
        min_heap.push(3);
        min_heap.push(1);
        min_heap.push(2);
        min_heap.delete(2); // 4 is removed
        assert_eq!(min_heap.peek(), Some(&1));
        min_heap.pop();
        assert_eq!(min_heap.peek(), Some(&2));
        min_heap.pop();
        assert_eq!(min_heap.peek(), Some(&3));
        min_heap.pop();
        assert_eq!(min_heap.peek(), Some(&7));
        min_heap.delete(1); // Index do not exists, nothing should happen
        assert_eq!(min_heap.peek(), Some(&7));
        min_heap.pop();
        assert!(min_heap.is_empty())
    }
}
