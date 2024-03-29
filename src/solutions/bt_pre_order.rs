#[allow(unused)]
use crate::solutions::tree::{BTree, make_test_tree};

#[allow(dead_code)]
#[allow(unused)]
fn pre_order_search(root: BTree) -> Vec<i32> {
    let mut path: Vec<i32> = vec![];
    walk(Some(&root), &mut path);
    path
}

fn walk(current: Option<&BTree>, path: &mut Vec<i32>) {
    match current {
        None => {}
        Some(current) => {
            // pre
            path.push(current.value);

            // recurse
            walk(current.left.as_deref(), path);
            walk(current.right.as_deref(), path);

            // post
        }
    }
}

#[test]
fn pre_order_search_test() {
    let tree = make_test_tree();
    assert_eq!(
        vec![20, 10, 5, 7, 15, 50, 30, 29, 45, 100],
        pre_order_search(tree)
    );
}

