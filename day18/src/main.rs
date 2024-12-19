use std::fs::File;
use std::io::prelude::*;

mod tools;
use tools::*;

const INPUT: &str = "input.txt";
const ROWS: usize = 71;
const COLS: usize = 71;

fn dijkstra(
    board: &[[char; COLS]; ROWS],
    start: (usize, usize),
    end: (usize, usize),
) -> Option<usize> {
    let mut pq: MinHeap<(usize, usize)> = MinHeap::new();
    let mut seen: [[bool; ROWS]; COLS] = [[false; ROWS]; COLS];

    pq.push(0, start);

    while let Some((current_cost, current_point)) = pq.pop() {

        if current_point == end {
            return Some(current_cost);
        }

        if seen[current_point.1][current_point.0] {
            continue;
        } else {
            seen[current_point.1][current_point.0] = true;
        }

        for dir in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let new_point = (
                current_point.0 as isize + dir.0,
                current_point.1 as isize + dir.1,
            );

            if !(new_point.0 >= 0
                && new_point.0 < ROWS as isize
                && new_point.1 >= 0
                && new_point.1 < COLS as isize)
            {
                continue;
            }

            let new_point = (new_point.0 as usize, new_point.1 as usize);

            if board[new_point.1][new_point.0] == '#' {
                continue;
            }

            let new_cost = current_cost + 1;

            pq.push(new_cost, new_point);
        }
    }

    None
}

fn main() {
    let drops = {
        let mut file: File = File::open(INPUT).expect("Failed to open INPUT file");
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read contents of INPUT file");

        let mut drops: Vec<Point> = Vec::new();

        for lines in contents.split("\n").filter(|l| l.len() > 0) {
            let (x, y) = lines.split_once(",").unwrap();
            drops.push(Point::from((x.parse().unwrap(), y.parse().unwrap())));
        }

        drops
    };

    let mut board: [[char; COLS]; ROWS] = [['.'; COLS]; ROWS];

    for (index, Point(x, y)) in drops.iter().enumerate() {
        board[*y][*x] = '#';

        if let Some(val) = dijkstra(&board, (0, 0), (70, 70)) {
            if index + 1 == 1024 {
                println!("{:?} - {:?}", index + 1, val);
            }
        } else {
            println!("Failed {:?} - {:?}", index + 1, (x, y));
            break;
        }
    }

    for y in 0..board.len() {
        for x in 0..board[y].len() {
            print!("{}", board[y][x]);
        }
        print!("\n");
    }
}
