#[allow(unused)]
use crate::solutions::tree::{make_test_tree1, make_test_tree2, BTree};

#[allow(dead_code)]
#[allow(unused)]
fn compare(a: &BTree, b: &BTree) -> bool {
    if a.value == b.value {
        return sub_compare(&a.left, &b.left) && sub_compare(&a.right, &b.right);
    }
    false
}

fn sub_compare(a: &Option<Box<BTree>>, b: &Option<Box<BTree>>) -> bool {
    match (a, b) {
        (None, Some(_)) => false,
        (Some(_), None) => false,
        (Some(a), Some(b)) => compare(a, b),
        (None, None) => true,
    }
}

#[test]
fn compare_test() {
    let tree1 = make_test_tree1();
    let tree2 = make_test_tree2();
    assert_eq!(true, compare(&tree1, &tree1));
    assert_eq!(false, compare(&tree1, &tree2));
}
