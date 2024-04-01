mod problems;
mod solutions;

use rand::seq::SliceRandom;

fn main() {
    let problem_names = vec![
        "linear_search",
        "binary_search",
        "two_crystal_balls",
        "bubble_sort",
        "maze_solver",
        "quick_sort",
        "bt_pre_order",
        "bt_in_order",
        "bt_post_order",
        "bt_bfs",
        "compare_binary_trees",
        "dfs_on_bst",
        "min_heap",
        "bfs_graph_matrix",
        "dfs_graph_list",
        "dijkstra_list",
    ];
    println!("All problems are {:?}.", problem_names);
    println!();

    let mut rng = rand::thread_rng();
    println!(
        "Try to implement {} in the problems directory.",
        problem_names.choose(&mut rng).unwrap()
    );
}
