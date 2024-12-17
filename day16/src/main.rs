use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::usize;

const INPUT: &str = "input.txt";
const INPUT_SIZE: usize = 141;
const DIRS: [Dir; 4] = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];
type MazeArr = [[Tile; INPUT_SIZE]; INPUT_SIZE];

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Wall,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Empty => ".",
            Self::Wall => "#",
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl std::fmt::Debug for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Up => "^",
            Self::Down => "v",
            Self::Left => "<",
            Self::Right => ">",
        })
    }
}

impl Dir {
    fn resolve(&self) -> (isize, isize) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
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

    fn difference(&self, other: Self) -> usize {
        let (r1, c1) = self.resolve();
        let (r2, c2) = other.resolve();

        (r2 - r1).abs().max((c2 - c1).abs()) as usize
    }
}

#[derive(Debug)]
struct Score(usize, (usize, usize), Dir);

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 .0 == other.1 .0 && self.1 .1 == other.1 .1 && self.2 == other.2
    }
}

impl Eq for Score {}

struct Maze {
    map: MazeArr,
    start: (usize, usize),
    end: (usize, usize),
}

impl std::fmt::Debug for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out: String = String::new();

        for row in 0..self.map.len() {
            for col in 0..self.map[row].len() {
                if row == self.start.0 && col == self.start.1 {
                    out += "S";
                } else if row == self.end.0 && col == self.end.1 {
                    out += "E";
                } else {
                    out += format!("{:?}", self.map[row][col]).as_str();
                }
            }

            out += "\n";
        }

        f.write_str(out.as_str())
    }
}

fn dijkstra(maze: &Maze) -> Option<usize> {
    let mut pq: BinaryHeap<Score> = BinaryHeap::new();
    let mut seen: HashSet<((usize, usize), Dir)> = HashSet::new();

    pq.push(Score(0, maze.start, Dir::Right));

    while pq.len() > 0 {
        let Some(Score(current_score, p, current_dir)) = pq.pop() else {
            break;
        };
        seen.insert((p, current_dir));

        if p == maze.end {
            return Some(current_score);
        }

        for dir in DIRS {
            let Some(new_p) = dir.apply(p) else {
                continue;
            };

            if maze.map[new_p.0][new_p.1] != Tile::Empty {
                continue;
            }

            if seen.contains(&(new_p, dir)) {
                continue;
            }

            let new_score = current_dir.difference(dir) * 1000 + 1 + current_score;

            pq.push(Score(new_score, new_p, dir));
        }
    }

    None
}

fn modified_dijkstra(maze: &Maze) -> usize {
    let mut pq: BinaryHeap<Score> = BinaryHeap::new();
    let mut lowest_cost: HashMap<((usize, usize), Dir), usize> = HashMap::new();
    let mut backtrack: HashMap<((usize, usize), Dir), HashSet<((usize, usize), Dir)>> =
        HashMap::new();

    pq.push(Score(0, maze.start, Dir::Right));

    let mut end_lowest_cost: usize = usize::MAX;
    let mut end_states: HashSet<((usize, usize), Dir)> = HashSet::new();

    while pq.len() > 0 {
        let Some(Score(current_score, p, current_dir)) = pq.pop() else {
            break;
        };

        if current_score > *lowest_cost.get(&(p, current_dir)).unwrap_or(&usize::MAX) {
            continue;
        }

        if p == maze.end {
            if current_score > end_lowest_cost {
                break;
            }
            end_lowest_cost = current_score;
            end_states.insert((p, current_dir));
        }

        for dir in DIRS {
            let Some(new_p) = dir.apply(p) else {
                continue;
            };

            if maze.map[new_p.0][new_p.1] != Tile::Empty {
                continue;
            }

            let &prev_lowest = lowest_cost.get(&(new_p, dir)).unwrap_or(&usize::MAX);

            let new_score = current_dir.difference(dir) * 1000 + 1 + current_score;

            if new_score > prev_lowest {
                continue;
            } else if new_score < prev_lowest {
                backtrack.remove(&(new_p, dir));
                lowest_cost.insert((new_p, dir), new_score);
            }

            if let Some(b) = backtrack.get_mut(&(new_p, dir)) {
                b.insert((p, current_dir));
            } else {
                let mut hs = HashSet::new();
                hs.insert((p, current_dir));
                backtrack.insert((new_p, dir), hs);
            }

            pq.push(Score(new_score, new_p, dir));
        }
    }

    let mut states: VecDeque<((usize, usize), Dir)> = VecDeque::new();
    states.extend(&end_states);

    let mut seen: HashSet<((usize, usize), Dir)> = HashSet::new();
    seen.extend(&end_states);

    let mut points: HashSet<(usize, usize)> = HashSet::new();
    points.insert(maze.end);

    while states.len() > 0 {
        let Some(key) = states.pop_front() else {
            continue;
        };
        let Some(set) = backtrack.get(&key) else {
            continue;
        };

        for &last in set.iter() {
            if seen.contains(&last) {
                continue;
            }
            seen.insert(last);
            states.push_back(last);
            points.insert(last.0);
        }
    }

    points.len()
}

fn main() {
    let result: Maze = {
        let mut file: File = File::open(INPUT).expect("Failed to open INPUT file");
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read contents of INPUT file");

        let mut map: MazeArr = [[Tile::Empty; INPUT_SIZE]; INPUT_SIZE];
        let mut start: (usize, usize) = (0, 0);
        let mut end: (usize, usize) = (0, 0);

        for (row, row_str) in contents.split("\n").filter(|c| c.len() > 0).enumerate() {
            for (col, ch) in row_str.chars().enumerate() {
                map[row][col] = match ch {
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    'E' => {
                        end = (row, col);
                        Tile::Empty
                    }
                    'S' => {
                        start = (row, col);
                        Tile::Empty
                    }
                    _ => panic!("Unrecognized input {:?}", ch),
                }
            }
        }

        Maze {
            map: map,
            start: start,
            end: end,
        }
    };

    println!("{:?}", result);
    let part1 = dijkstra(&result);
    println!("Part 1 - {:?}", part1);
    let part2 = modified_dijkstra(&result);
    println!("Part 2 - {:?}", part2);
}
