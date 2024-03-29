use crate::solutions::tree::{make_test_tree1, make_test_tree2, BTree};

fn compare(a: &BTree, b: &BTree) -> bool {
    if a.value != b.value {
        return false;
    }
    let left_compare = match (a.left.as_deref(), b.left.as_deref()) {
        (None, None) => true,
        (Some(_), None) => false, // Could merge this and the next into (_,_). Same below
        (None, Some(_)) => false,
        (Some(a_left), Some(b_left)) => compare(a_left, b_left),
    };
    if left_compare {
        return match (a.right.as_deref(), b.right.as_deref()) {
            (None, None) => true,
            (Some(_), None) => false,
            (None, Some(_)) => false,
            (Some(a_right), Some(b_right)) => compare(a_right, b_right),
        };
    }
    false
}

#[test]
fn bfs_search_test() {
    let tree1 = make_test_tree1();
    let tree2 = make_test_tree2();
    assert_eq!(true, compare(&tree1, &tree1));
    assert_eq!(false, compare(&tree1, &tree2));
}
