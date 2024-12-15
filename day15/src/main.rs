use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

const INPUT: &str = "input.txt";
const INPUT_SIZE: usize = 50;

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Box,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Empty => ".",
            Self::Wall => "#",
            Self::Box => "O",
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum BigTile {
    Empty,
    Wall,
    LeftBox,
    RightBox,
}

impl std::fmt::Debug for BigTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Empty => ".",
            Self::Wall => "#",
            Self::LeftBox => "[",
            Self::RightBox => "]",
        })
    }
}

#[derive(Clone, Copy, Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn resolve(&self) -> (isize, isize) {
        match self {
            Move::Up => (-1, 0),
            Move::Down => (1, 0),
            Move::Left => (0, -1),
            Move::Right => (0, 1),
        }
    }

    fn apply(&self, (r, c): (usize, usize)) -> Option<(usize, usize)> {
        let (dr, dc) = self.resolve();
        let (ur, uc) = (r as isize, c as isize);
        let (new_r, new_c) = (ur + dr, uc + dc);

        if new_r >= 0 && new_r < INPUT_SIZE as isize && new_c >= 0 && new_c < INPUT_SIZE as isize {
            Some((new_r as usize, new_c as usize))
        } else {
            None
        }
    }

    fn apply_big(&self, (r, c): (usize, usize)) -> Option<(usize, usize)> {
        let (dr, dc) = self.resolve();
        let (ur, uc) = (r as isize, c as isize);
        let (new_r, new_c) = (ur + dr, uc + dc);

        if new_r >= 0 && new_r < INPUT_SIZE as isize && new_c >= 0 && new_c < (INPUT_SIZE*2) as isize {
            Some((new_r as usize, new_c as usize))
        } else {
            None
        }
    }
}

struct Warehouse {
    map: [[Tile; INPUT_SIZE]; INPUT_SIZE],
    robot: (usize, usize),
    moves: Vec<Move>,
}

impl std::fmt::Debug for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out: String = String::new();

        for row in 0..self.map.len() {
            for col in 0..self.map[row].len() {
                if (row, col) == self.robot {
                    out += "@";
                } else {
                    out += format!("{:?}", self.map[row][col]).as_str();
                }
            }

            out += "\n";
        }

        f.write_str(out.as_str())
    }
}

impl Warehouse {
    fn shuffle(&mut self, pos: (usize, usize), item: Tile, dir: Move) -> bool {
        let current_item = self.map[pos.0][pos.1];
    
        match current_item {
            Tile::Empty => {
                self.map[pos.0][pos.1] = item;
                true
            },
            Tile::Wall => false,
            Tile::Box => {
                let Some(next_pos) = dir.apply(pos) else { return false; };
    
                let next_shuffle = self.shuffle(next_pos, current_item, dir);
    
                if next_shuffle {
                    self.map[pos.0][pos.1] = item;
                    true
                } else {
                    false
                }
            },
        }
    }

    fn move_robot(&mut self, dir: Move) {
        let Some((new_r, new_c)) = dir.apply(self.robot) else { return; };
        let move_onto_tile: Tile = self.map[new_r][new_c];

        match move_onto_tile {
            Tile::Empty => { self.robot = (new_r, new_c); },
            Tile::Wall => { return },
            Tile::Box => { let shuffle_result = self.shuffle((new_r, new_c), Tile::Empty, dir);
            if shuffle_result {
                self.robot = (new_r, new_c);
            } },
        }
    }

    fn gps(&self) -> usize {
        let mut acc: usize = 0;

        for row in 0..self.map.len() {
            for col in 0..self.map[row].len() {
                if let Tile::Box = self.map[row][col] {
                    acc += row * 100 + col;
                }
            }
        }

        acc
    }

    fn complete_moves(&mut self) {
        for mv in self.moves.clone() {
            self.move_robot(mv);
        }
    }
}

struct BigWarehouse {
    map: [[BigTile; INPUT_SIZE*2]; INPUT_SIZE],
    robot: (usize, usize),
    moves: Vec<Move>,
}

impl std::fmt::Debug for BigWarehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out: String = String::new();

        for row in 0..self.map.len() {
            for col in 0..self.map[row].len() {
                if (row, col) == self.robot {
                    out += "@";
                } else {
                    out += format!("{:?}", self.map[row][col]).as_str();
                }
            }

            out += "\n";
        }

        f.write_str(out.as_str())
    }
}

impl BigWarehouse {
    fn new(wh: &Warehouse) -> BigWarehouse {
        let mut map: [[BigTile; INPUT_SIZE*2]; INPUT_SIZE] = [[BigTile::Empty; INPUT_SIZE*2]; INPUT_SIZE];

        for row in 0..wh.map.len() {
            for col in 0..wh.map[row].len() {
                let (left, right) = match wh.map[row][col] {
                    Tile::Empty => (BigTile::Empty, BigTile::Empty),
                    Tile::Wall => (BigTile::Wall, BigTile::Wall),
                    Tile::Box => (BigTile::LeftBox, BigTile::RightBox),
                };

                map[row][2*col] = left;
                map[row][2*col+1] = right;
            }
        }

        BigWarehouse {
            map: map,
            robot: (wh.robot.0, wh.robot.1*2),
            moves: wh.moves.clone(),
        }
    }

    fn up_down_scan(&self, pos: (usize, usize), move_set: &mut HashSet<((usize, usize), (usize, usize), BigTile)>, dir: Move) -> bool {
        let current_tile = self.map[pos.0][pos.1];
        let Some(next_pos) = dir.apply_big(pos) else { return false; };
        let next_tile = self.map[next_pos.0][next_pos.1];
        
        move_set.insert((pos, next_pos, current_tile));

        match next_tile {
            BigTile::Empty => true,
            BigTile::Wall => false,
            BigTile::LeftBox => {
                let left_scan = self.up_down_scan(next_pos, move_set, dir);
                let right_box = (next_pos.0, next_pos.1 + 1);
                let right_scan = self.up_down_scan(right_box, move_set, dir);

                if left_scan && right_scan {
                    true
                } else {
                    false
                }
            },
            BigTile::RightBox => {
                let right_scan = self.up_down_scan(next_pos, move_set, dir);
                let left_box = (next_pos.0, next_pos.1 - 1);
                let left_scan = self.up_down_scan(left_box, move_set, dir);

                if left_scan && right_scan {
                    true
                } else {
                    false
                }
            },
        }
    }

    fn move_tiles(&mut self, move_set: &HashSet<((usize, usize), (usize, usize), BigTile)>) {
        for (from, _, _) in move_set.iter() {
            self.map[from.0][from.1] = BigTile::Empty;
        }

        for (_, to, tile) in move_set.iter() {
            self.map[to.0][to.1] = *tile;
        }
    }

    fn shuffle(&mut self, pos: (usize, usize), item: BigTile, dir: Move) -> bool {
        let current_item = self.map[pos.0][pos.1];

    
        match dir {
            Move::Left | Move::Right => {
                match current_item {
                    BigTile::Empty => { self.map[pos.0][pos.1] = item;
                        true },
                    BigTile::Wall => false,
                    BigTile::LeftBox | BigTile::RightBox => {
                        let Some(next_pos) = dir.apply_big(pos) else { return false; };
        
                    let next_shuffle = self.shuffle(next_pos, current_item, dir);
        
                    if next_shuffle {
                        self.map[pos.0][pos.1] = item;
                        true
                    } else {
                        false
                    }
                    },
                }
            },
            Move::Up | Move::Down => {
                match current_item {
                    BigTile::Empty => { self.map[pos.0][pos.1] = item;
                        true },
                    BigTile::Wall => false,
                    BigTile::LeftBox => {
                        let mut move_set: HashSet<((usize, usize), (usize, usize), BigTile)> = HashSet::new();
                        let left_result = self.up_down_scan(pos, &mut move_set, dir);
                        let right_result = self.up_down_scan((pos.0, pos.1 + 1), &mut move_set, dir);
    
                        if !(left_result && right_result) {
                            return false;
                        }
    
                        self.move_tiles(&move_set);
    
                        true
                    },
                    BigTile::RightBox => {
                        let mut move_set: HashSet<((usize, usize), (usize, usize), BigTile)> = HashSet::new();
                        let left_result = self.up_down_scan((pos.0, pos.1 - 1), &mut move_set, dir);
                        let right_result = self.up_down_scan(pos, &mut move_set, dir);
    
                        if !(left_result && right_result) {
                            return false;
                        }
    
                        self.move_tiles(&move_set);
    
                        true
                    },
                }
            },
        }
    }

    fn move_robot(&mut self, dir: Move) {
        let Some((new_r, new_c)) = dir.apply_big(self.robot) else { return; };
        let move_onto_tile: BigTile = self.map[new_r][new_c];

        match move_onto_tile {
            BigTile::Empty => { self.robot = (new_r, new_c); },
            BigTile::Wall => { return },
            BigTile::LeftBox | BigTile::RightBox => {
                let shuffle_result = self.shuffle((new_r, new_c), BigTile::Empty, dir);

                if shuffle_result {
                    self.robot = (new_r, new_c);
                } 
            },
        }
    }

    fn gps(&self) -> usize {
        let mut acc: usize = 0;

        for row in 0..self.map.len() {
            for col in 0..self.map[row].len() {
                if let BigTile::LeftBox = self.map[row][col] {
                    acc += row * 100 + col;
                }
            }
        }

        acc
    }

    fn complete_moves(&mut self) {
        for mv in self.moves.clone() {
            self.move_robot(mv);
            //println!("{:?} \n {:?}", mv, self);
        }
    }
}

fn main() {
    let mut warehouse: Warehouse = {
        let mut file: File = File::open(INPUT).expect("Failed to open INPUT file");
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read contents of INPUT file");

        let (board, movements) = contents.split_once("\n\n").unwrap();

        let mut warehouse: [[Tile; INPUT_SIZE]; INPUT_SIZE] = [[Tile::Empty; INPUT_SIZE]; INPUT_SIZE];
        let mut robot = (0, 0);

        for (row, row_str) in board.split("\n").filter(|l| l.len() > 0).enumerate() {
            for (col, tile) in row_str.chars().enumerate() {
                warehouse[row][col] = match tile {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    'O' => Tile::Box,
                    '@' => {robot = (row, col); Tile::Empty},
                    _ => panic!("Unrecognized warehouse input"),
                }
            }
        }

        let mut moves: Vec<Move> = Vec::new();

        for mv in movements.chars().filter(|c| *c != '\n') {
            moves.push(match mv {
                '^' => Move::Up,
                'v' => Move::Down,
                '<' => Move::Left,
                '>' => Move::Right,
                _ => panic!("Unrecognized move {:?}", mv),
            });
        }

        Warehouse {
            map: warehouse,
            robot: robot,
            moves: moves,
        }
    };

    let mut big_warehouse: BigWarehouse = BigWarehouse::new(&warehouse);

    println!("{:?}", warehouse);
    warehouse.complete_moves();
    println!("{:?}", warehouse);
    println!("Part 1 - Normal Warehouse GPS: {:?}\n", warehouse.gps());

    println!("{:?}", big_warehouse);
    big_warehouse.complete_moves();
    println!("{:?}", big_warehouse);
    println!("Part 2 - Big Warehouse GPS: {:?}\n", big_warehouse.gps());
}
