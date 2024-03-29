#[allow(unused)]
use crate::solutions::tree::{make_test_tree1, make_test_tree2, BTree};
#[allow(unused)]
fn compare(a: &BTree, b: &BTree) -> bool {
    todo!()
}

#[test]
fn bfs_search_test() {
    let tree1 = make_test_tree1();
    let tree2 = make_test_tree2();
    assert_eq!(true, compare(&tree1, &tree1));
    assert_eq!(false, compare(&tree1, &tree2));
}
