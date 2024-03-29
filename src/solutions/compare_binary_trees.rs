#[allow(unused)]
use crate::solutions::tree::{make_test_tree1, make_test_tree2, BTree};

#[allow(dead_code)]
#[allow(unused)]
fn compare(a: &BTree, b: &BTree) -> bool {
    compare_option(Some(a), Some(b))
}

fn compare_option(a: Option<&BTree>, b: Option<&BTree>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(_), None) => false,
        (None, Some(_)) => false,
        (Some(a), Some(b)) => {
            if a.value != b.value {
                return false;
            }
            compare_option(a.left.as_deref(), b.left.as_deref())
                && compare_option(a.right.as_deref(), b.right.as_deref())
        }
    }
}

#[test]
fn bfs_search_test() {
    let tree1 = make_test_tree1();
    let tree2 = make_test_tree2();
    assert_eq!(true, compare(&tree1, &tree1));
    assert_eq!(false, compare(&tree1, &tree2));
}
