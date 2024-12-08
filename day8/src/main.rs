use std::cmp::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

const INPUT: &str = "input.txt";

struct TowerMap {
    towers: HashMap<char, Vec<(usize, usize)>>,
    rows: usize,
    cols: usize,
}

impl TowerMap {
    fn combinations(list: &Vec<(usize, usize)>) -> Vec<((usize, usize), (usize, usize))> {
        let mut combos: Vec<((usize, usize), (usize, usize))> = Vec::new();

        for i in 0..list.len() {
            for j in 0 + i + 1..list.len() {
                combos.push((list[i], list[j]));
            }
        }

        combos
    }

    fn antinode_pair(
        &self,
        p1: (usize, usize),
        p2: (usize, usize),
        set: &mut HashSet<(usize, usize)>,
    ) {
        let p1: (isize, isize) = (
            isize::try_from(p1.0).unwrap(),
            isize::try_from(p1.1).unwrap(),
        );
        let p2: (isize, isize) = (
            isize::try_from(p2.0).unwrap(),
            isize::try_from(p2.1).unwrap(),
        );

        let dr: isize = p2.0 - p1.0;
        let dc: isize = p2.1 - p1.1;

        let an1: (isize, isize) = (p2.0 + dr, p2.1 + dc);
        let an2: (isize, isize) = (p1.0 - dr, p1.1 - dc);

        let an1 = (usize::try_from(an1.0), usize::try_from(an1.1));
        let an2 = (usize::try_from(an2.0), usize::try_from(an2.1));

        if let (Ok(r), Ok(c)) = an1 {
            if r <= self.rows && c <= self.cols {
                set.insert((r, c));
            }
        }

        if let (Ok(r), Ok(c)) = an2 {
            if r <= self.rows && c <= self.cols {
                set.insert((r, c));
            }
        }
    }

    fn all_antinodes(&self) -> HashSet<(usize, usize)> {
        let mut set: HashSet<(usize, usize)> = HashSet::new();

        for v in self.towers.values() {
            let combos = TowerMap::combinations(v);

            for (p1, p2) in combos.iter() {
                self.antinode_pair(*p1, *p2, &mut set);
            }
        }

        set
    }

    fn antinode_ray(
        &self,
        p1: (usize, usize),
        p2: (usize, usize),
        set: &mut HashSet<(usize, usize)>,
        loop_limit: usize
    ) {
        set.insert(p2);

        let p1: (isize, isize) = (
            isize::try_from(p1.0).unwrap(),
            isize::try_from(p1.1).unwrap(),
        );
        let p2: (isize, isize) = (
            isize::try_from(p2.0).unwrap(),
            isize::try_from(p2.1).unwrap(),
        );

        let dr: isize = p2.0 - p1.0;
        let dc: isize = p2.1 - p1.1;

        let mut last_node: (isize, isize) = p2;
        let mut iterations: usize = 0;

        loop {
            if iterations >= loop_limit { break; }

            let an: (isize, isize) = (last_node.0 + dr, last_node.1 + dc);

            let an_usize = (usize::try_from(an.0), usize::try_from(an.1));

            if let (Ok(r), Ok(c)) = an_usize {
                if r <= self.rows && c <= self.cols {
                    set.insert((r, c));
                    last_node = an;
                } else {
                    break;
                }
            } else {
                break;
            }

            iterations += 1;
        }
    }

    fn resonant_antinodes(&self) -> HashSet<(usize, usize)> {
        let mut set: HashSet<(usize, usize)> = HashSet::new();

        for v in self.towers.values() {
            let combos = TowerMap::combinations(v);

            for (p1, p2) in combos.iter() {
                self.antinode_ray(*p1, *p2, &mut set, 1000);
                self.antinode_ray(*p2, *p1, &mut set, 1000);
            }
        }

        set
    }
}

fn main() {
    let towers: TowerMap = {
        let mut file: File = File::open(INPUT).expect("Failed to open INPUT file");
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read contents of INPUT file");

        let mut towers: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
        let mut rows: usize = 0;
        let mut cols: usize = 0;

        for (row, row_text) in contents.split("\n").filter(|r| r.len() > 0).enumerate() {
            for (col, ch) in row_text.chars().enumerate() {
                if ch != '.' {
                    if let Some(v) = towers.get_mut(&ch) {
                        v.push((row, col));
                    } else {
                        let mut v: Vec<(usize, usize)> = Vec::new();
                        v.push((row, col));
                        towers.insert(ch, v);
                    }
                }

                cols = max(cols, col);
            }

            rows = max(rows, row);
        }

        TowerMap {
            towers: towers,
            rows: rows,
            cols: cols,
        }
    };

    let antinodes = towers.all_antinodes();
    println!("Part 1 - Antinodes: {:?}", antinodes.len());
    let resonant_antinodes = towers.resonant_antinodes();
    println!("Part 2 - Antinodes: {:?}", resonant_antinodes.len());
}
