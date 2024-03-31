#[allow(unused)]
use crate::solutions::graph::{list2, WeightedAdjacencyListItem};

#[allow(dead_code)]
#[allow(unused)]
/**
 * DFS search on a graph returning the found path.
 * The graph is represented as a weighted adjacency list item.
 * Complexity: O(|V| + |E|) since in the worst case each node and edge are checked once.
 */
fn dfs(
    graph: &[Vec<WeightedAdjacencyListItem>],
    source: usize,
    needle: usize,
) -> Option<Vec<usize>> {
    let mut path = vec![];
    if walk(
        graph,
        source,
        needle,
        &mut vec![false; graph.len()],
        &mut path,
    ) {
        return Some(path);
    }
    None
}

fn walk(
    graph: &[Vec<WeightedAdjacencyListItem>],
    current: usize,
    needle: usize,
    seen: &mut Vec<bool>,
    path: &mut Vec<usize>,
) -> bool {
    if seen[current] {
        return false;
    }
    seen[current] = true;
    path.push(current);
    if current == needle {
        return true;
    }
    for edge in &graph[current] {
        if walk(graph, edge.to, needle, seen, path) {
            return true;
        }
    }
    path.pop();
    false
}

#[test]
fn bfs_test() {
    let list2 = list2();
    assert_eq!(Some(vec![0, 1, 4, 5, 6,]), dfs(&list2, 0, 6));
    assert_eq!(None, dfs(&list2, 6, 0));
}
