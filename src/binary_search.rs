#[allow(dead_code)]
/**
 * Return the index of the element v if it exists using binary search.
 * The input vector must be ordered for binary search.
 * O(log n) time complexity.
 */
fn binary_search(haystack: Vec<i32>, needle: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = haystack.len();
    while low < high {
        let middle = low + (high - low) / 2;
        let value = *haystack.get(middle).unwrap(); // Must always exist
        if value == needle {
            return Some(middle);
        }
        if value > needle {
            high = middle;
        } else {
            low = middle + 1;
        }
    }
    None
}

#[test]
fn binary_search_test() {
    assert_eq!(None, binary_search(vec![], 3));
    assert_eq!(None, binary_search(vec![1, 2, 3, 4, 5], 7));

    assert_eq!(Some(0), binary_search(vec![1, 2, 3, 4, 5], 1));
    assert_eq!(Some(2), binary_search(vec![1, 2, 3, 4, 5], 3));
    assert_eq!(Some(4), binary_search(vec![1, 2, 3, 4, 5], 5));
}
