#[allow(unused)]
struct MinHeap<T: Ord> {
    heap: Vec<T>,
}
#[allow(unused)]
#[allow(dead_code)]
impl<T: Ord> MinHeap<T> {
    fn new() -> MinHeap<T> {
        MinHeap { heap: vec![] }
    }

    fn len(&self) -> usize {
        self.heap.len()
    }

    fn insert(&mut self, value: T) {
        todo!();
    }

    fn delete(&mut self) -> T {
        todo!();
    }
}

#[test]
fn min_heap_test() {
    let mut heap = MinHeap::new();

    assert_eq!(heap.len(), 1);

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
