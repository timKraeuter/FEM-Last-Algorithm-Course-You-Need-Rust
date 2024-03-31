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
    pub weight: usize,
}
#[allow(unused)]
pub fn list1() -> Vec<Vec<WeightedAdjacencyListItem>> {
    vec![
        vec![
            WeightedAdjacencyListItem { to: 1, weight: 3 },
            WeightedAdjacencyListItem { to: 2, weight: 1 },
        ],
        vec![
            WeightedAdjacencyListItem { to: 0, weight: 3 },
            WeightedAdjacencyListItem { to: 2, weight: 4 },
            WeightedAdjacencyListItem { to: 4, weight: 1 },
        ],
        vec![
            WeightedAdjacencyListItem { to: 1, weight: 4 },
            WeightedAdjacencyListItem { to: 3, weight: 7 },
            WeightedAdjacencyListItem { to: 0, weight: 1 },
        ],
        vec![
            WeightedAdjacencyListItem { to: 2, weight: 7 },
            WeightedAdjacencyListItem { to: 4, weight: 5 },
            WeightedAdjacencyListItem { to: 6, weight: 1 },
        ],
        vec![
            WeightedAdjacencyListItem { to: 1, weight: 1 },
            WeightedAdjacencyListItem { to: 3, weight: 5 },
            WeightedAdjacencyListItem { to: 5, weight: 2 },
        ],
        vec![
            WeightedAdjacencyListItem { to: 6, weight: 1 },
            WeightedAdjacencyListItem { to: 4, weight: 2 },
            WeightedAdjacencyListItem { to: 2, weight: 18 },
        ],
        vec![
            WeightedAdjacencyListItem { to: 3, weight: 1 },
            WeightedAdjacencyListItem { to: 5, weight: 1 },
        ],
    ]
}

pub fn list2() -> Vec<Vec<WeightedAdjacencyListItem>> {
    vec![
        vec![
            WeightedAdjacencyListItem { to: 1, weight: 3 },
            WeightedAdjacencyListItem { to: 2, weight: 1 },
        ],
        vec![WeightedAdjacencyListItem { to: 4, weight: 1 }],
        vec![WeightedAdjacencyListItem { to: 3, weight: 7 }],
        vec![],
        vec![
            WeightedAdjacencyListItem { to: 1, weight: 1 },
            WeightedAdjacencyListItem { to: 3, weight: 5 },
            WeightedAdjacencyListItem { to: 5, weight: 2 },
        ],
        vec![
            WeightedAdjacencyListItem { to: 2, weight: 18 },
            WeightedAdjacencyListItem { to: 6, weight: 1 },
        ],
        vec![WeightedAdjacencyListItem { to: 3, weight: 1 }],
    ]
}
