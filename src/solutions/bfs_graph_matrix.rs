#[allow(unused)]
use crate::solutions::graph::matrix;
use std::collections::VecDeque;

#[allow(dead_code)]
#[allow(unused)]
/**
 * BFS search on a graph returning the found path.
 * The graph is represented as an adjacency matrix.
 */
fn bfs(graph: &[Vec<usize>], source: usize, needle: usize) -> Option<Vec<usize>> {
    let mut seen: Vec<bool> = vec![false; graph.len()];
    seen[source] = true;
    let mut previous: Vec<i32> = vec![-1; graph.len()];

    let mut deque = VecDeque::new();
    deque.push_back(source);

    while !deque.is_empty() {
        let current = deque.pop_front().expect("Deque is not empty.");
        if current == needle {
            break;
        }
        for (i, weight) in graph[current].iter().enumerate() {
            if *weight == 0 {
                // weight 0 means no edge exists.
                continue;
            }
            if seen[i] {
                continue;
            }
            seen[i] = true;
            previous[i] = current as i32;
            deque.push_back(i);
        }
    }
    if !seen[needle] {
        return None;
    }
    // Build path
    let mut path = vec![needle];
    let mut current = needle;
    while current != source {
        current = previous[current] as usize; // Potentially dangerous cast but the path should always exist.
        path.push(current);
    }
    path.reverse();
    Some(path)
}

#[test]
fn bfs_test() {
    let matrix = matrix();
    assert_eq!(Some(vec![0, 1, 4, 5, 6,]), bfs(&matrix, 0, 6));
    assert_eq!(None, bfs(&matrix, 6, 0));
}
