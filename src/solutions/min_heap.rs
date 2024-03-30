#[allow(unused)]
struct MinHeap<T: Ord> {
    heap: Vec<T>,
}
#[allow(unused)]
impl<T: Ord> MinHeap<T> {
    pub fn new() -> MinHeap<T> {
        MinHeap { heap: vec![] }
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn insert(&mut self, value: T) {
        self.heap.push(value);
        self.heapify_up(self.heap.len() - 1)
    }

    fn heapify_up(&mut self, current: usize) {
        if current == 0 {
            // Iteratively would be faster but this codes nicer.
            return;
        }
        let parent_index = Self::parent_index(current);
        if self.heap[current] < self.heap[parent_index] {
            self.heap.swap(current, parent_index);
            self.heapify_up(parent_index);
        }
    }

    fn heapify_down(&mut self, current: usize) {
        // Swap stuff down recursively.
        let left_child = Self::left_child_index(current);
        if current >= self.heap.len() || left_child >= self.heap.len() {
            return;
        }
        let right_child = Self::right_child_index(current);
        let left_value = &self.heap[left_child];
        let current_value = &self.heap[current];
        // This check is not in the course since JS just returns undefined when out of bounds for an array.
        // Then undefined > x or x > undefined returns false. Rust throws an error instead.
        if right_child >= self.heap.len() && current_value > left_value {
            // Only left exists.
            self.heap.swap(current, left_child);
            return;
        }
        let right_value = &self.heap[right_child];

        if left_value > right_value && current_value > right_value {
            self.heap.swap(current, right_child);
            self.heapify_down(right_child);
        } else if right_value > left_value && current_value > left_value {
            self.heap.swap(current, left_child);
            self.heapify_down(left_child);
        }
    }

    pub fn delete(&mut self) -> T {
        if self.heap.is_empty() {
            panic!("Cannot remove from an empty heap!");
        }
        let last_idx = self.heap.len() - 1;
        self.heap.swap(0, last_idx);
        let min = self.heap.pop();

        self.heapify_down(0);

        min.unwrap()
    }

    fn parent_index(idx: usize) -> usize {
        if idx == 0 {
            return 0;
        }
        (idx - 1) / 2
    }

    fn left_child_index(current: usize) -> usize {
        current * 2 + 1
    }

    fn right_child_index(current: usize) -> usize {
        current * 2 + 2
    }
}

#[test]
fn min_heap_test() {
    let mut heap = MinHeap::new();

    assert_eq!(heap.len(), 0);

    heap.insert(5);
    heap.insert(3);
    heap.insert(69);
    heap.insert(420);
    heap.insert(4);
    heap.insert(1);
    heap.insert(8);
    heap.insert(7);

    assert_eq!(heap.len(), 8);
    assert_eq!(heap.delete(), 1);
    assert_eq!(heap.delete(), 3);
    assert_eq!(heap.delete(), 4);
    assert_eq!(heap.delete(), 5);
    assert_eq!(heap.len(), 4);
    assert_eq!(heap.delete(), 7);
    assert_eq!(heap.delete(), 8);
    assert_eq!(heap.delete(), 69);
    assert_eq!(heap.delete(), 420);
    assert_eq!(heap.len(), 0);
}
