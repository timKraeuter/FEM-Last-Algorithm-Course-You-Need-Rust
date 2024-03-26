#[allow(dead_code)]
#[allow(unused)]
// Given two crystal balls that will break if dropped from high enough
// distance, determine the exact spot in which it will break in the most
// optimized way.

fn two_crystal_balls(breaks: Vec<bool>) -> usize {
    todo!();
}

#[test]
fn linear_search_test() {
    assert_eq!(0, two_crystal_balls(vec![true, true, true]));
    assert_eq!(1, two_crystal_balls(vec![false, true, true]));
    assert_eq!(2, two_crystal_balls(vec![false, false, true]));
    assert_eq!(
        3,
        two_crystal_balls(vec![false, false, false, true, true, true])
    );
    assert_eq!(
        14,
        two_crystal_balls(vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, true, true, true
        ])
    );
}
