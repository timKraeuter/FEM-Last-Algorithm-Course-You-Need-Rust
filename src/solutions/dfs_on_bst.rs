#[allow(unused)]
use crate::solutions::tree::{make_test_tree1, BTree};
#[allow(unused)]
fn dfs(head: &BTree, needle: i32) -> bool {
    search(Some(head), needle)
}

fn search(current: Option<&BTree>, needle: i32) -> bool {
    match current {
        None => false,
        Some(current) => {
            if current.value == needle {
                return true;
            }
            if needle <= current.value {
                return search(current.left.as_deref(), needle);
            }
            return search(current.right.as_deref(), needle);
        }
    }
}

#[test]
fn dfs_test() {
    let tree = make_test_tree1();
    assert_eq!(true, dfs(&tree, 45));
    assert_eq!(true, dfs(&tree, 7));
    assert_eq!(false, dfs(&tree, 69));
}
