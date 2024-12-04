use std::fs::File;
use std::io::prelude::*;

const INPUT: &str = "input.txt";
const PUZZLE_SIZE: usize = 140;

type PuzzleBoard = [[char; PUZZLE_SIZE]; PUZZLE_SIZE];

enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    fn change(&self, (row, col): (usize, usize)) -> Option<(usize, usize)> {
        let row: isize = row
            .try_into()
            .expect("Directions.change failed to convert row");
        let col: isize = col
            .try_into()
            .expect("Directions.change failed to convert col");

        let (new_row, new_col): (isize, isize) = match self {
            Direction::Up => (row - 1, col),
            Direction::UpRight => (row - 1, col + 1),
            Direction::Right => (row, col + 1),
            Direction::DownRight => (row + 1, col + 1),
            Direction::Down => (row + 1, col),
            Direction::DownLeft => (row + 1, col - 1),
            Direction::Left => (row, col - 1),
            Direction::UpLeft => (row - 1, col - 1),
        };

        let new_row = usize::try_from(new_row);
        let new_col = usize::try_from(new_col);

        match (new_row, new_col) {
            (Ok(r), Ok(c)) => {
                let safe_values: bool = r < PUZZLE_SIZE && c < PUZZLE_SIZE; // r >= 0 && c >= 0 implicit by types

                if safe_values {
                    Some((r, c))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

fn parse_input() -> PuzzleBoard {
    let mut file: File = File::open(INPUT).expect("Failed to open file");
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read contents");

    let mut puzzle: PuzzleBoard = [['\0'; PUZZLE_SIZE]; PUZZLE_SIZE];

    let chars: Vec<Vec<char>> = contents
        .split("\n")
        .filter(|&s| s.len() > 0)
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    for (row, row_vec) in chars.iter().enumerate() {
        for (col, ch) in row_vec.iter().enumerate() {
            puzzle[row][col] = ch.to_owned();
        }
    }

    puzzle
}

fn ray_match(arr: &PuzzleBoard, (row, col): (usize, usize), dir: &Direction, key: &str) -> bool {
    if key.len() <= 0 {
        false
    } else if key.len() <= 1 {
        arr[row][col].to_string() == key
    } else {
        if let Some((new_row, new_col)) = dir.change((row, col)) {
            let new_key = &key[1..];

            key.chars().collect::<Vec<_>>()[0] == arr[row][col]
                && ray_match(arr, (new_row, new_col), dir, new_key)
        } else {
            false
        }
    }
}

fn part_1(puzzle: &PuzzleBoard) {
    const SEARCH_DIRS: [Direction; 8] = [
        Direction::Up,
        Direction::UpRight,
        Direction::Right,
        Direction::DownRight,
        Direction::Down,
        Direction::DownLeft,
        Direction::Left,
        Direction::UpLeft,
    ];

    const SEARCH_TERM: &str = "XMAS";
    let mut count = 0;

    for row in 0..puzzle.len() {
        for col in 0..puzzle.len() {
            for dir in SEARCH_DIRS.iter() {
                if ray_match(&puzzle, (row, col), &dir, SEARCH_TERM) {
                    count += 1;
                }
            }
        }
    }

    println!("Part 1 - XMAS count {:?}", count);
}

fn part_2(puzzle: &PuzzleBoard) {
    fn point_translate(
        row: usize,
        col: usize,
    ) -> Option<(
        (usize, usize),
        (usize, usize),
        (usize, usize),
        (usize, usize),
    )> {
        let down_left = Direction::DownLeft.change((row, col))?;
        let up_left = Direction::UpLeft.change((row, col))?;
        let down_right = Direction::DownRight.change((row, col))?;
        let up_right = Direction::UpRight.change((row, col))?;

        Some((down_left, up_left, down_right, up_right))
    }

    const SEARCH_TERM: &str = "MAS";
    let mut count = 0;
    let mut arm_1: bool;
    let mut arm_2: bool;

    for row in 0..puzzle.len() {
        for col in 0..puzzle.len() {
            if let Some((down_left, up_left, down_right, up_right)) = point_translate(row, col) {
                // One direction on each arm of the X needs to have the search term

                arm_1 = ray_match(&puzzle, up_left, &Direction::DownRight, SEARCH_TERM)
                    ^ ray_match(&puzzle, down_right, &Direction::UpLeft, SEARCH_TERM);

                arm_2 = ray_match(&puzzle, down_left, &Direction::UpRight, SEARCH_TERM)
                    ^ ray_match(&puzzle, up_right, &Direction::DownLeft, SEARCH_TERM);
            } else {
                arm_1 = false;
                arm_2 = false;
            }

            if arm_1 && arm_2 {
                count += 1;
            }
        }
    }

    println!("Part 2 - X-MAS count {:?}", count);
}

fn main() {
    println!("Hello, world!");
    let puzzle: PuzzleBoard = parse_input();
    part_1(&puzzle);
    part_2(&puzzle);
}
