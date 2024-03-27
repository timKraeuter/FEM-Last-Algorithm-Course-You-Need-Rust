#[allow(unused)]
/**
 * Find the path from the start to the end in the given maze. See example in the test below
 */
fn solve(maze: Vec<&str>, wall: &str, start: Point, end: Point) -> Vec<Point> {
    let mut seen = vec![];
    let mut path = vec![];

    walk(&maze, wall, &start, &end, &mut seen, &mut path);

    path
}

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn walk(
    maze: &Vec<&str>,
    wall: &str,
    current: &Point,
    end: &Point,
    seen: &mut Vec<Point>,
    path: &mut Vec<Point>,
) -> bool {
    // Going of the map.
    if current.y < 0
        || current.y >= maze.len() as i32
        || current.x < 0
        || current.x >= maze[0].len() as i32
    {
        return false;
    }
    // Hitting a wall
    if get_symbol_for_point(&maze, &current) == wall {
        return false;
    }
    // Finding the end
    if current == end {
        path.push(current.clone());
        return true;
    }
    // Seeing something where we have been before.
    if seen.contains(current) {
        return false;
    }

    // pre
    seen.push(current.clone());
    path.push(current.clone());

    // recurse
    for (x_offset, y_offset) in DIRECTIONS {
        let new_point = Point {
            x: current.x + x_offset,
            y: current.y + y_offset,
        };
        if walk(maze, wall, &new_point, end, seen, path) {
            return true;
        }
    }
    // post
    path.pop();

    false
}

fn get_symbol_for_point<'a>(maze: &Vec<&'a str>, point: &Point) -> &'a str {
    let row = maze[point.y as usize];
    let x = point.x as usize;
    &row[x..x + 1]
}

#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[test]
fn get_symbol_for_point_test() {
    let maze = vec![
        "xxxxxxxxxxSx", // Start here
        "x        x x",
        "x        x x",
        "x xxxxxxxx x",
        "x          x",
        "xExxxxxxxxxx", // End here on the left
    ];
    let start = Point { x: 10, y: 0 };
    assert_eq!("S", get_symbol_for_point(&maze, &start));

    let end = Point { x: 1, y: 5 };
    assert_eq!("E", get_symbol_for_point(&maze, &end));
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
