#[allow(unused)]
pub fn matrix() -> Vec<Vec<usize>> {
    vec![
        vec![0, 3, 1, 0, 0, 0, 0], // 0
        vec![0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 7, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 5, 0, 2, 0],
        vec![0, 0, 18, 0, 0, 0, 1],
        vec![0, 0, 0, 1, 0, 0, 1],
    ]
}

pub struct WeightedAdjacencyListItem {
    pub to: usize,
    pub weight: f64, // We use f64 for distances since this leads to less casting in dijkstra in Rust since infinity only exists for f64.
}
#[allow(unused)]
pub fn list1() -> Vec<Vec<WeightedAdjacencyListItem>> {
    vec![
        vec![
            WeightedAdjacencyListItem {
                to: 1,
                weight: 3f64,
            },
            WeightedAdjacencyListItem {
                to: 2,
                weight: 1f64,
            },
        ],
        vec![
            WeightedAdjacencyListItem {
                to: 0,
                weight: 3f64,
            },
            WeightedAdjacencyListItem {
                to: 2,
                weight: 4f64,
            },
            WeightedAdjacencyListItem {
                to: 4,
                weight: 1f64,
            },
        ],
        vec![
            WeightedAdjacencyListItem {
                to: 1,
                weight: 4f64,
            },
            WeightedAdjacencyListItem {
                to: 3,
                weight: 7f64,
            },
            WeightedAdjacencyListItem {
                to: 0,
                weight: 1f64,
            },
        ],
        vec![
            WeightedAdjacencyListItem {
                to: 2,
                weight: 7f64,
            },
            WeightedAdjacencyListItem {
                to: 4,
                weight: 5f64,
            },
            WeightedAdjacencyListItem {
                to: 6,
                weight: 1f64,
            },
        ],
        vec![
            WeightedAdjacencyListItem {
                to: 1,
                weight: 1f64,
            },
            WeightedAdjacencyListItem {
                to: 3,
                weight: 5f64,
            },
            WeightedAdjacencyListItem {
                to: 5,
                weight: 2f64,
            },
        ],
        vec![
            WeightedAdjacencyListItem {
                to: 6,
                weight: 1f64,
            },
            WeightedAdjacencyListItem {
                to: 4,
                weight: 2f64,
            },
            WeightedAdjacencyListItem {
                to: 2,
                weight: 18f64,
            },
        ],
        vec![
            WeightedAdjacencyListItem {
                to: 3,
                weight: 1f64,
            },
            WeightedAdjacencyListItem {
                to: 5,
                weight: 1f64,
            },
        ],
    ]
}

pub fn list2() -> Vec<Vec<WeightedAdjacencyListItem>> {
    vec![
        vec![
            WeightedAdjacencyListItem {
                to: 1,
                weight: 3f64,
            },
            WeightedAdjacencyListItem {
                to: 2,
                weight: 1f64,
            },
        ],
        vec![WeightedAdjacencyListItem {
            to: 4,
            weight: 1f64,
        }],
        vec![WeightedAdjacencyListItem {
            to: 3,
            weight: 7f64,
        }],
        vec![],
        vec![
            WeightedAdjacencyListItem {
                to: 1,
                weight: 1f64,
            },
            WeightedAdjacencyListItem {
                to: 3,
                weight: 5f64,
            },
            WeightedAdjacencyListItem {
                to: 5,
                weight: 2f64,
            },
        ],
        vec![
            WeightedAdjacencyListItem {
                to: 2,
                weight: 18f64,
            },
            WeightedAdjacencyListItem {
                to: 6,
                weight: 1f64,
            },
        ],
        vec![WeightedAdjacencyListItem {
            to: 3,
            weight: 1f64,
        }],
    ]
}
