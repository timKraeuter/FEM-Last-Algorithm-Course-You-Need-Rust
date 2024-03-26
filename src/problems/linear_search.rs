#[allow(dead_code)]
#[allow(unused)]
/**
 * Return the index of the element v if it exists using linear search.
 */
fn linear_search(haystack: Vec<i32>, needle: i32) -> Option<usize> {
    todo!();
}

#[test]
fn linear_search_test() {
    assert_eq!(None, linear_search(vec![], 3));

    assert_eq!(None, linear_search(vec![1, 2, 3, 4, 5], 7));
    assert_eq!(Some(2), linear_search(vec![1, 22, 3, 4, 8], 3));
}
