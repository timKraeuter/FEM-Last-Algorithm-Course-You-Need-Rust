#[allow(unused)]
use crate::solutions::graph::matrix;
#[allow(dead_code)]
#[allow(unused)]
/**
 * BFS search on a graph returning the found path.
 * The graph is represented as an adjacency matrix.
 */
fn bfs(graph: &[Vec<usize>], source: usize, needle: usize) -> Option<Vec<usize>> {
    todo!();
}

#[test]
fn bfs_test() {
    let matrix = matrix();
    assert_eq!(Some(vec![0, 1, 4, 5, 6,]), bfs(&matrix, 0, 6));
    assert_eq!(None, bfs(&matrix, 6, 0));
}
