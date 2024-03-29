#[allow(unused)]
use crate::solutions::tree::{make_test_tree, BTree};
use std::collections::VecDeque;
#[allow(dead_code)]
#[allow(unused)]
fn bfs(root: &BTree, needle: i32) -> bool {
    let mut queue = VecDeque::new();
    queue.push_back(root);

    while !queue.is_empty() {
        let node = queue.pop_front().expect("Cannot be empty");
        if node.value == needle {
            return true;
        }
        if let Some(left) = node.left.as_deref() {
            queue.push_back(left)
        }
        if let Some(right) = node.right.as_deref() {
            queue.push_back(right)
        }
    }
    false
}

#[test]
fn bfs_search_test() {
    let tree = make_test_tree();
    assert_eq!(true, bfs(&tree, 45));
    assert_eq!(true, bfs(&tree, 7));
    assert_eq!(false, bfs(&tree, 69));
}
