#[allow(unused)]
use crate::solutions::graph::{list1, WeightedAdjacencyListItem};

#[allow(unused)]
fn dijkstra(graph: &[Vec<WeightedAdjacencyListItem>], source: usize, sink: usize) -> Vec<usize> {
    let mut seen = vec![false; graph.len()];
    let mut previous = vec![-1; graph.len()]; // ok to just put 0?
    let mut distances = vec![f64::INFINITY; graph.len()];
    distances[source] = 0f64;

    while has_unvisited(&seen, &distances) {
        let current = get_lowest_unvisited(&seen, &distances);
        seen[current] = true;
        for edge in graph[current].iter() {
            if seen[edge.to] {
                continue;
            }
            let dist = distances[current] + edge.weight;
            if dist < distances[edge.to] {
                distances[edge.to] = dist;
                previous[edge.to] = current as i32;
            }
        }
    }
    let mut out = vec![];
    let mut current = sink;
    while previous[current] != -1 {
        out.push(current);
        current = previous[current] as usize;
    }
    out.push(source);
    out.reverse();
    out
}

fn get_lowest_unvisited(seen: &Vec<bool>, distances: &Vec<f64>) -> usize {
    let mut lowest_distance = f64::INFINITY;
    let mut idx = 0;
    for (index, seen) in seen.iter().enumerate() {
        if *seen {
            continue;
        }
        if lowest_distance > distances[index] {
            lowest_distance = distances[index];
            idx = index;
        }
    }
    idx
}

fn has_unvisited(seen: &Vec<bool>, distances: &Vec<f64>) -> bool {
    for (index, seen) in seen.iter().enumerate() {
        if !*seen && distances[index] < f64::INFINITY {
            return true;
        }
    }
    false
}

#[test]
fn dijkstra_test() {
    let list1 = list1();
    assert_eq!(vec![0, 1, 4, 5, 6,], dijkstra(&list1, 0, 6));
}
