#[allow(dead_code)]
#[allow(unused)]
/**
 * Find the path from the start to the end in the given maze. See example in the test below
 */
fn solve(maze: Vec<&str>, wall: &str, start: Point, end: Point) -> Vec<Point> {
    todo!();
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[test]
fn solve_test() {
    let path = solve(
        vec![
            "xxxxxxxxxxSx", // Start here
            "x        x x",
            "x        x x",
            "x xxxxxxxx x",
            "x          x",
            "xExxxxxxxxxx", // End here on the left
        ],
        "x",
        Point { x: 10, y: 0 },
        Point { x: 1, y: 5 },
    );
    assert_eq!(
        vec![
            Point { x: 10, y: 0 },
            Point { x: 10, y: 1 },
            Point { x: 10, y: 2 },
            Point { x: 10, y: 3 },
            Point { x: 10, y: 4 },
            Point { x: 9, y: 4 },
            Point { x: 8, y: 4 },
            Point { x: 7, y: 4 },
            Point { x: 6, y: 4 },
            Point { x: 5, y: 4 },
            Point { x: 4, y: 4 },
            Point { x: 3, y: 4 },
            Point { x: 2, y: 4 },
            Point { x: 1, y: 4 },
            Point { x: 1, y: 5 },
        ],
        path
    );
}
