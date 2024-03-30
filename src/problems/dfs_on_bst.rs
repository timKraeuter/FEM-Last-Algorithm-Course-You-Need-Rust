#[allow(unused)]
use crate::solutions::tree::{make_test_tree1, BTree};
#[allow(unused)]
fn dfs(head: &BTree, needle: i32) -> bool {
    todo!()
}

#[test]
fn dfs_test() {
    let tree = make_test_tree1();
    assert_eq!(true, dfs(&tree, 45));
    assert_eq!(true, dfs(&tree, 7));
    assert_eq!(false, dfs(&tree, 69));
}
