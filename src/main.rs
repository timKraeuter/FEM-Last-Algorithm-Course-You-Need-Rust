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
    ];
    println!("All problems are {:?}.", problem_names);
    println!();

    let mut rng = rand::thread_rng();
    println!(
        "Try to implement {} in the problems directory.",
        problem_names.choose(&mut rng).unwrap()
    );
}
