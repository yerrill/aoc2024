use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::prelude::*;

const INPUT: &str = "input.txt";

const MAX_DEPTH: usize = 4; // 3 for part 1, 26 for part 2

const KEYPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A'],
];

const DIRPAD: [[char; 3]; 2] = [[' ', '^', 'A'], ['<', 'v', '>']];

const DIRS: [(isize, isize, char); 4] = [(-1, 0, '^'), (0, 1, '>'), (1, 0, 'v'), (0, -1, '<')];

fn bfs_movements<const R: usize, const C: usize>(
    arr: [[char; C]; R],
    from: char,
    to: char,
) -> Vec<Vec<char>> {
    let mut valid_paths: Vec<Vec<char>> = Vec::new();

    let mut queue: VecDeque<((usize, usize), Vec<char>)> = VecDeque::new();

    let mut from_index = None;

    for row in 0..R {
        for col in 0..C {
            if arr[row][col] == from {
                from_index = Some((row, col));
                break;
            }
        }
    }

    let from_index: (usize, usize) = from_index.expect("Could not find char from");

    queue.push_back((from_index, Vec::new()));

    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    while let Some((point, list)) = queue.pop_front() {
        seen.insert(point);

        if arr[point.0][point.1] == to {
            let mut mod_list = list;
            mod_list.push('A');
            valid_paths.push(mod_list);
            continue;
        }

        for (dr, dc, ch) in DIRS {
            let nr = point.0 as isize + dr;
            let nc = point.1 as isize + dc;

            if !(nr >= 0 && nr < R as isize && nc >= 0 && nc < C as isize) {
                continue;
            }

            let new_point = (nr as usize, nc as usize);

            if arr[new_point.0][new_point.1] == ' ' {
                continue;
            }

            if seen.contains(&new_point) {
                continue;
            }

            let mut new_list = list.clone();
            new_list.push(ch);
            queue.push_back((new_point, new_list));
        }
    }

    let shortest_path: usize = valid_paths.iter().map(|p| p.len()).min().unwrap_or(0);

    let shortest_paths: Vec<Vec<char>> = valid_paths
        .iter()
        .filter(|p| p.len() <= shortest_path)
        .map(|p| p.clone())
        .collect();

    shortest_paths
}

fn movement(
    code: Vec<char>,
    depth: usize,
    cache: &mut HashMap<(char, char, usize), Vec<char>>,
) -> Vec<char> {
    if depth <= 0 {
        return code;
    }

    let mut cumulative: Vec<char> = Vec::new();

    let mut best_section: Vec<char>;
    let mut best_section_len: usize;
    let mut test_path: Vec<char>;
    let mut paths: Vec<Vec<char>>;

    let mut last_char: char = 'A';

    for ch in code {
        if let Some(v) = cache.get(&(last_char, ch, depth)) {
            cumulative.append(&mut v.clone());
            last_char = ch;
            continue;
        }

        if depth >= MAX_DEPTH {
            paths = bfs_movements(KEYPAD, last_char, ch);
        } else {
            paths = bfs_movements(DIRPAD, last_char, ch);
        }

        best_section = Vec::new();
        best_section_len = usize::MAX;

        for path in paths {
            test_path = movement(path.clone(), depth - 1, cache);

            if test_path.len() < best_section_len {
                best_section = test_path;
                best_section_len = best_section.len();
            }
        }

        cache.insert((last_char, ch, depth), best_section.clone());

        cumulative.append(&mut best_section);
        last_char = ch;
    }

    cumulative
}

fn movement2(
    code: Vec<char>,
    depth: usize,
    cache: &mut HashMap<(char, char, usize), usize>,
) -> usize {
    if depth <= 0 {
        return code.len();
    }

    let mut accumulator: usize = 0;

    let mut best_section_len: usize;
    let mut test_path_len: usize;
    let mut paths: Vec<Vec<char>>;

    let mut last_char: char = 'A';

    for ch in code {
        if let Some(v) = cache.get(&(last_char, ch, depth)) {
            accumulator += v;
            last_char = ch;
            continue;
        }

        if depth >= MAX_DEPTH {
            paths = bfs_movements(KEYPAD, last_char, ch);
        } else {
            paths = bfs_movements(DIRPAD, last_char, ch);
        }

        best_section_len = usize::MAX;

        for path in paths {
            test_path_len = movement2(path.clone(), depth - 1, cache);

            if test_path_len < best_section_len {
                best_section_len = test_path_len;
            }
        }

        cache.insert((last_char, ch, depth), best_section_len);

        accumulator += best_section_len;
        last_char = ch;
    }

    accumulator
}

fn main() {
    let codes = {
        let mut file: File = File::open(INPUT).expect("Failed to open INPUT file");
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read contents of INPUT file");

        let mut codes: Vec<Vec<char>> = Vec::new();

        for line in contents.split("\n").filter(|l| l.len() > 0) {
            codes.push(line.chars().collect::<Vec<char>>());
        }

        codes
    };

    let mut result: Vec<char>;
    let mut result2: usize;
    let mut num: usize;
    let mut acc: usize = 0;
    let mut acc2: usize = 0;

    for code in codes {
        num = (&code[0..3]
            .iter()
            .map(|ch| ch.to_string())
            .collect::<Vec<_>>()
            .join(""))
            .parse()
            .unwrap();

        if MAX_DEPTH < 5 {
            result = movement(code.clone(), MAX_DEPTH, &mut HashMap::new());
            acc += num * result.len();
            println!("{:?} {:?}", result, result.len());

            result2 = movement2(code.clone(), MAX_DEPTH, &mut HashMap::new());
            acc2 += num * result2;
        } else {
            result2 = movement2(code.clone(), MAX_DEPTH, &mut HashMap::new());
            acc2 += num * result2;
        }
    }

    println!("{:?} {:?}", acc, acc2);
}
