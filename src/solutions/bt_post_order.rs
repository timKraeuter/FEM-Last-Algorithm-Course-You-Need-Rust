#[allow(unused)]
use crate::solutions::tree::{make_test_tree, BTree};

#[allow(dead_code)]
#[allow(unused)]
fn post_order_search(root: BTree) -> Vec<i32> {
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
            walk(current.right.as_deref(), path);

            // post
            path.push(current.value);
        }
    }
}

#[test]
fn post_order_search_test() {
    let tree = make_test_tree();
    assert_eq!(
        vec![7, 5, 15, 10, 29, 45, 30, 100, 50, 20],
        post_order_search(tree)
    );
}
