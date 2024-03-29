#[allow(unused)]
use crate::solutions::tree::{make_test_tree1, BTree};

#[allow(dead_code)]
#[allow(unused)]
fn in_order_search(root: BTree) -> Vec<i32> {
    let mut path: Vec<i32> = vec![];
    walk(Some(&root), &mut path);
    path
}

fn walk(current: Option<&BTree>, path: &mut Vec<i32>) {
    match current {
        None => {}
        Some(current) => {
            // pre

            // recurse
            walk(current.left.as_deref(), path);
            path.push(current.value);
            walk(current.right.as_deref(), path);

            // post
        }
    }
}

#[test]
fn in_order_search_test() {
    let tree = make_test_tree1();
    assert_eq!(
        vec![5, 7, 10, 15, 20, 29, 30, 45, 50, 100],
        in_order_search(tree)
    );
}
