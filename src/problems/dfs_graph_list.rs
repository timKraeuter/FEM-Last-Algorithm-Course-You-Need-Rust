#[allow(unused)]
use crate::solutions::graph::{list2, WeightedAdjacencyListItem};

#[allow(dead_code)]
#[allow(unused)]
/**
 * DFS search on a graph returning the found path.
 * The graph is represented as a weighted adjacency list.
 */
fn dfs(
    graph: &[Vec<WeightedAdjacencyListItem>],
    source: usize,
    needle: usize,
) -> Option<Vec<usize>> {
    todo!();
}

#[test]
fn dfs_test() {
    let list2 = list2();
    assert_eq!(Some(vec![0, 1, 4, 5, 6,]), dfs(&list2, 0, 6));
    assert_eq!(None, dfs(&list2, 6, 0));
}
