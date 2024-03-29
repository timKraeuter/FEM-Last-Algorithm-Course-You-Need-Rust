#[allow(unused)]
use crate::solutions::tree::{make_test_tree1, BTree};

#[allow(dead_code)]
#[allow(unused)]
fn bfs(root: &BTree, needle: i32) -> bool {
    todo!();
}

#[test]
fn bfs_search_test() {
    let tree = make_test_tree1();
    assert_eq!(true, bfs(&tree, 45));
    assert_eq!(true, bfs(&tree, 7));
    assert_eq!(false, bfs(&tree, 69));
}
