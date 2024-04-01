#[allow(unused)]
use crate::solutions::graph::{list1, WeightedAdjacencyListItem};

#[allow(dead_code)]
#[allow(unused)]
fn dijkstra(
    graph: &[Vec<WeightedAdjacencyListItem>],
    source: usize,
    sink: usize,
) -> Option<Vec<usize>> {
    todo!()
}

#[test]
fn dijkstra_test() {
    let list1 = list1();
    assert_eq!(Some(vec![0, 1, 4, 5, 6,]), dijkstra(&list1, 0, 6));
}
