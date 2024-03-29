#[allow(unused)]
use crate::solutions::tree::{make_test_tree1, BTree};

#[allow(dead_code)]
#[allow(unused)]
fn pre_order_search(root: BTree) -> Vec<i32> {
    vec![]
}

#[test]
fn pre_order_search_test() {
    let tree = make_test_tree1();
    assert_eq!(
        vec![20, 10, 5, 7, 15, 50, 30, 29, 45, 100],
        pre_order_search(tree)
    );
}
