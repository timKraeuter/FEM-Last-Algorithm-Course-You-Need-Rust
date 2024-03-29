#[allow(unused)]
use crate::solutions::tree::{make_test_tree, BTree};

#[allow(dead_code)]
#[allow(unused)]
fn in_order_search(root: BTree) -> Vec<i32> {
    vec![]
}

#[test]
fn in_order_search_test() {
    let tree = make_test_tree();
    assert_eq!(
        vec![5, 7, 10, 15, 20, 29, 30, 45, 50, 100],
        in_order_search(tree)
    );
}
