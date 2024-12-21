use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;

mod tools;
use tools::*;

const INPUT: &str = "input.txt";
const ROWS: usize = 15;
const COLS: usize = 15;
const DOUBLE_MOVE_CHEAT: [(isize, isize); 12] = [
    (-3, 0),
    (-2, 1),
    (-1, 2),
    (0, 3),
    (1, 2),
    (2, 1),
    (3, 0),
    (2, -1),
    (1, -2),
    (0, -3),
    (-1, -2),
    (-2, -1),
];
const SINGLE_MOVE_CHEAT: [(isize, isize); 8] = [
    (-2, 0),
    (-1, 1),
    (0, 2),
    (1, 1),
    (2, 0),
    (1, -1),
    (0, -2),
    (-1, -1),
];

type MazeBoard = Board<Option<usize>, ROWS, COLS>;

/*
fn find_cheats(board: &MazeBoard, pos: Point) -> HashSet<(Point, Point, usize)> {
    let mut set: HashSet<(Point, Point, usize)> = HashSet::new();
    let value_at_pos: isize = board.at(pos).unwrap().unwrap() as isize;

    for singles in SINGLE_MOVE_CHEAT {
        let Some(new_pos) = board.translate_point(pos, singles) else {
            continue;
        }; // Point is in bounds
        let Some(Some(value_at_new_pos)) = board.at(new_pos) else {
            continue;
        }; // Can get point and point is on path

        let saved_time: isize = *value_at_new_pos as isize - value_at_pos - 2;

        if saved_time > 0 {
            set.insert((pos, new_pos, saved_time as usize));
        }
    }

    for doubles in DOUBLE_MOVE_CHEAT {
        let Some(new_pos) = board.translate_point(pos, doubles) else {
            continue;
        }; // Point is in bounds
        let Some(Some(value_at_new_pos)) = board.at(new_pos) else {
            continue;
        }; // Can get point and point is on path

        let saved_time: isize = *value_at_new_pos as isize - value_at_pos - 3;

        if saved_time > 0 {
            set.insert((pos, new_pos, saved_time as usize));
        }
    }

    set
}*/


fn create_modified(board: &MazeBoard, pos: Point, dir: (isize, isize)) -> MazeBoard {
    let mut modified = board.clone();
    let mut point = pos;

    for i in 0..2 {
        if let Some(new_p) = board.translate_point(point, dir) {
            modified.set(new_p, Some(0));
            point = new_p;
        } else {
            break;
        }
    }

    modified
}

fn find_cheats2(board: &MazeBoard, start: Point, end: Point) -> HashSet<(Point, Point, usize)> {
    const CHEAT_DIRS_ONE: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    const CHEAT_DIRS_TWO: [(isize, isize); 4] = [(-2, 0), (0, 2), (2, 0), (0, -2)];

    let (base_time, _) = board.dijkstras(0, start, end, |v| v.is_some());
    let mut new_time: usize;

    let mut set: HashSet<(Point, Point, usize)> = HashSet::new();

    let mut pos = start;

    let mut modified: MazeBoard;

    for i in 0..4 {
        let point_one = board.translate_point(from, delta)
        modified = create_modified(board, pos, dir.resolve());
        (new_time, _) = modified.dijkstras(0, start, end, |v| v.is_some());
        new_time = base_time - new_time;

        if new_time > 0 {
            set.insert(pos, )
        }
    }

    set
}

fn main() {
    let (mut board, start, end) = {
        let mut file: File = File::open(INPUT).expect("Failed to open INPUT file");
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read contents of INPUT file");

        let mut arr: [[Option<usize>; 15]; 15] = [[None; COLS]; ROWS];
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

        (Board { arr: arr }, start, end)
    };

    println!("{:?} {:?}", start, end);

    let (end_cost, mut board) = board.dijkstras(0, start, end, |v| v.is_some());

    println!(
        "{}",
        board.string_using(|val| match val {
            Some(v) => v.to_string(),
            None => String::from("."),
        })
    );

    println!("{:?}", find_cheats(&board, start));
}
