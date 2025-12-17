use crate::parser::parse_file_from_day;

#[derive(Debug)]
enum Point {
    ROLL,
    SPACE
}

fn create_graph(input: Vec<String>) -> Vec<Vec<Point>> {
    let mut tp_graph: Vec<Vec<Point>> = Vec::new();
    for line in input {
        let mut cols: Vec<Point> = Vec::new();
        for char in line.chars() {
            match char {
                '.' => cols.push(Point::SPACE),
                '@' => cols.push(Point::ROLL),
                _ => panic!("Invalid input: {}", char)
            };
        }
        tp_graph.push(cols);
    }

    tp_graph
}

fn adjacent_roll_count(graph: &Vec<Vec<Point>>, target_x: isize, target_y: isize) -> usize {
    let mut roll_count = 0;
    for x in target_x - 1..=target_x + 1 {
        for y in target_y - 1..=target_y + 1 {
            if x < 0 || y < 0 || x >= graph.len().cast_signed() || (target_x == x && target_y == y) {
                continue;
            }

            let row = &graph[x as usize];
            if y >= row.len().cast_signed() {
                continue;
            }

            match row[y as usize] {
                Point::ROLL => roll_count += 1,
                _ => (),
            }
        }
    }

    roll_count
}

pub(crate) fn solution1(is_test: bool) -> usize {
    let input = parse_file_from_day(4, is_test);
    let tp_graph = create_graph(input);
    let mut accessible_rolls = 0;

    for (x, row) in tp_graph.iter().enumerate() {
        for (y, point) in row.iter().enumerate() {
            match point {
                Point::ROLL => {
                    if adjacent_roll_count(&tp_graph, x as isize, y as isize) < 4 {
                        accessible_rolls += 1;
                    }
                },
                _ => continue
            }
        }
    }

    accessible_rolls
}

pub(crate) fn solution2(is_test: bool) -> usize {
    let input = parse_file_from_day(4, is_test);
    let mut tp_graph = create_graph(input);
    let mut accessible_rolls = 0;

    loop {
        let mut removable_rolls = Vec::new();
        for (x, row) in tp_graph.iter().enumerate() {
            for (y, point) in row.iter().enumerate() {
                match point {
                    Point::ROLL => {
                        let adjacent_rolls = adjacent_roll_count(&tp_graph, x as isize, y as isize);
                        if adjacent_rolls < 4 {
                            accessible_rolls += 1;
                            removable_rolls.push((x, y));
                        }
                    },
                    _ => continue
                }
            }
        }

        if removable_rolls.len() == 0 {
            break;
        }
        for (x, y) in removable_rolls {
            tp_graph[x][y] = Point::SPACE;
        }
    }

    accessible_rolls
}