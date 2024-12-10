use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

const INPUT: &str = "input.txt";
const ROWS: usize = 50;
const COLS: usize = 50;
const ROWS_SIGNED: isize = 50;
const COLS_SIGNED: isize = 50;
const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

type Arr = [[usize; COLS]; ROWS];
type Visited = [[bool; COLS]; ROWS];
type Cache = [[Option<usize>; COLS]; ROWS];

fn in_bounds(r: isize, c: isize) -> bool {
    r >= 0 && c >= 0 && r < ROWS_SIGNED && c < COLS_SIGNED
}

fn debug_arr(arr: &Arr) -> String {
    let mut result: String = String::new();

    for r in 0..ROWS {
        for c in 0..COLS {
            result += arr[r][c].to_string().as_str();
        }
        result += "\n";
    }

    result
}

fn debug_cache(arr: &Cache) -> String {
    let mut result: String = String::new();

    for r in 0..ROWS {
        for c in 0..COLS {
            let val = arr[r][c];
            if let Some(v) = val {
                result += v.to_string().as_str();
            } else {
                result += ".";
            }
        }
        result += "\n";
    }

    result
}

fn reset_visited(visit: &mut Visited) {
    for r in 0..ROWS {
        for c in 0..COLS {
            visit[r][c] = false;
        }
    }
}

fn find_peaks(arr: &Arr, visit: &mut Visited, peaks: &mut HashSet<(usize, usize)>, (r, c): (usize, usize)) {
    let value = arr[r][c];

    if value >= 9 {
        peaks.insert((r, c));
        return;
    }

    // Early return for already visited values
    if visit[r][c] {
        return;
    } else {
        visit[r][c] = true;
    }

    // Search directions
    for (dr, dc) in DIRS {
        let new_r = isize::try_from(r).expect("Failed to convert to isize") + dr;
        let new_c = isize::try_from(c).expect("Failed to convert to isize") + dc;

        if in_bounds(new_r, new_c) {
            let new_r = usize::try_from(new_r).expect("Failed to convert to usize");
            let new_c = usize::try_from(new_c).expect("Failed to convert to usize");

            if arr[new_r][new_c] == value + 1 {
                find_peaks(arr, visit, peaks, (new_r, new_c));
            }
        }
    }

    //visit[r][c] = false;
}

fn find_paths(arr: &Arr, cache: &mut Cache, (r, c): (usize, usize)) -> usize {
    let mut paths = 0;
    let value = arr[r][c];

    if value >= 9 {
        return 1;
    }

    // Early return for cached paths
    if let Some(p) = cache[r][c] {
        return p;
    }

    // Search directions
    for (dr, dc) in DIRS {
        let new_r = isize::try_from(r).expect("Failed to convert to isize") + dr;
        let new_c = isize::try_from(c).expect("Failed to convert to isize") + dc;

        if in_bounds(new_r, new_c) {
            let new_r = usize::try_from(new_r).expect("Failed to convert to usize");
            let new_c = usize::try_from(new_c).expect("Failed to convert to usize");

            if arr[new_r][new_c] == value + 1 {
                paths += find_paths(arr, cache, (new_r, new_c));
            }
        }
    }

    cache[r][c] = Some(paths);

    paths
}

fn main() {
    let topo: Arr = {
        let mut file: File = File::open(INPUT).expect("Failed to open INPUT file");
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read contents of INPUT file");

        let mut topo_input: Arr = [[0; COLS]; ROWS];

        for (r, row) in contents.split("\n").filter(|l| l.len() > 0).enumerate() {
            for (c, val) in row.chars().enumerate() {
                topo_input[r][c] = val.to_string().parse().expect("Failed to parse");
            }
        }

        topo_input
    };

    let mut visit: Visited = [[false; COLS]; ROWS];
    let mut cache: Cache = [[None; COLS]; ROWS];
    let mut peaks_acc: usize = 0;
    let mut paths_acc: usize = 0;

    for r in 0..ROWS {
        for c in 0..COLS {
            if topo[r][c] == 0 {
                reset_visited(&mut visit);
                let mut peaks: HashSet<(usize, usize)> = HashSet::new(); 
                
                find_peaks(&topo, &mut visit, &mut peaks, (r, c));
                peaks_acc += peaks.len();

                paths_acc += find_paths(&topo, &mut cache, (r, c));
            }
        }
    }

    println!("{}", debug_arr(&topo));
    println!("{}", debug_cache(&cache));
    println!("Part 1 - Trail Peak Score {:?}", peaks_acc);
    println!("Part 2 - Trail Paths Score {:?}", paths_acc);
}
