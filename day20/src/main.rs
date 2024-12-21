use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

mod tools;
use tools::*;

const INPUT: &str = "input.txt";
const ROWS: usize = 141;
const COLS: usize = 141;
const THRESHOLD: isize = 100;

type MazeBoard = Board<Option<usize>, ROWS, COLS>;

fn get_surrounding_points(board: &MazeBoard, start: Point, distance: isize) -> HashSet<Point> {
    let mut row: isize;
    let mut col: isize;
    let mut valid_points: HashSet<Point> = HashSet::new();

    for i in 0..distance + 1 {
        row = i;
        col = distance - row;

        for mult in [(1, 1), (-1, 1), (1, -1), (-1, -1)] {
            let Some(new_point) = board.translate_point(start, (row * mult.0, col * mult.1)) else {
                continue;
            };
            if board.is_valid_point(new_point) {
                valid_points.insert(new_point);
            }
        }
    }

    valid_points
}

fn find_cheats(board: &MazeBoard) -> usize {
    let mut count: usize = 0;

    for row in 0..ROWS {
        for col in 0..COLS {
            let start_point = Point(row, col);
            let Some(Some(start_val)) = board.at(start_point) else {
                continue;
            };

            for end_point in get_surrounding_points(board, start_point, 2) {
                let Some(Some(end_val)) = board.at(end_point) else {
                    continue;
                };

                if *end_val as isize - *start_val as isize >= THRESHOLD + 2 {
                    count += 1;
                }
            }
        }
    }

    count
}

fn find_cheats2(board: &MazeBoard) -> usize {
    let mut count: usize = 0;

    for row in 0..ROWS {
        for col in 0..COLS {
            let start_point = Point(row, col);
            let Some(Some(start_val)) = board.at(start_point) else {
                continue;
            };

            for distance in 2..21 {
                for end_point in get_surrounding_points(board, start_point, distance) {
                    let Some(Some(end_val)) = board.at(end_point) else {
                        continue;
                    };

                    if *end_val as isize - *start_val as isize >= THRESHOLD + distance {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn main() {
    let (board, start, end) = {
        let mut file: File = File::open(INPUT).expect("Failed to open INPUT file");
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read contents of INPUT file");

        let mut arr: Vec<Vec<Option<usize>>> = vec![vec![None; COLS]; ROWS];
        let mut start: Point = Point(0, 0);
        let mut end: Point = Point(0, 0);

        for (row, row_str) in contents.split("\n").filter(|l| l.len() > 0).enumerate() {
            for (col, ch) in row_str.chars().enumerate() {
                arr[row][col] = match ch {
                    '#' => None,
                    _ => Some(0),
                };

                if ch == 'S' {
                    start = Point(row, col);
                } else if ch == 'E' {
                    end = Point(row, col);
                }
            }
        }

        (Board::new(arr), start, end)
    };

    let (_, board) = board.dijkstras(0, start, end, |v| v.is_some());

    println!("Part 1 - Cheats: {:?}", find_cheats(&board));
    println!("Part 2 - Cheats: {:?}", find_cheats2(&board));
}
