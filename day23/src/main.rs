use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

const INPUT: &str = "input.txt";

type NetMap = HashMap<String, HashSet<String>>;

fn part1(nm: &NetMap) -> usize {
    let mut found_sets: HashSet<(String, String, String)> = HashSet::new();

    for x in nm.keys() {
        let Some(x_conns) = nm.get(x) else {
            continue;
        };

        for y in x_conns {
            let Some(y_conns) = nm.get(y) else {
                continue;
            };

            for z in y_conns {
                let Some(z_conns) = nm.get(z) else {
                    continue;
                };
                if x != z && z_conns.contains(x) {
                    let mut set = [x.to_string(), y.to_string(), z.to_string()];
                    set.sort();
                    found_sets.insert((set[0].clone(), set[1].clone(), set[2].clone()));
                }
            }
        }
    }

    found_sets
        .iter()
        .filter(|&s| s.0.starts_with("t") || s.1.starts_with("t") || s.2.starts_with("t"))
        .count()
}

fn bron_kerbosch(
    r: HashSet<String>,
    p: HashSet<String>,
    x: HashSet<String>,
    nm: &NetMap,
    results: &mut Vec<HashSet<String>>,
) {
    if p.len() <= 0 && x.len() <= 0 {
        results.push(r.clone());
        return;
    }

    let mut p = p;
    let mut x = x;

    for v in p.clone() {
        let Some(n_v) = nm.get(&v) else {
            continue;
        };

        let new_r = r.union(&HashSet::from([v.clone()])).cloned().collect();
        let new_p = p.intersection(n_v).cloned().collect();
        let new_x = p.intersection(n_v).cloned().collect();

        bron_kerbosch(new_r, new_p, new_x, nm, results);

        p.remove(&v);
        x.insert(v.to_string());
    }
}

fn part2(nm: &NetMap) -> String {
    let mut results = Vec::new();

    bron_kerbosch(
        HashSet::new(),
        HashSet::from_iter(nm.keys().cloned()),
        HashSet::new(),
        &nm,
        &mut results,
    );

    let largest = results.iter().max_by(|a, b| a.len().cmp(&b.len())).unwrap();

    let mut sorted = largest.iter().cloned().collect::<Vec<_>>();
    sorted.sort();

    sorted.join(",")
}

fn main() {
    let network_map: NetMap = {
        let mut file: File = File::open(INPUT).expect("Failed to open INPUT file");
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read contents of INPUT file");

        let mut network_map: HashMap<String, HashSet<String>> = HashMap::new();

        for line in contents.split("\n").filter(|l| l.len() > 0) {
            let (term1, term2) = line.split_once("-").unwrap();

            network_map
                .entry(term1.to_string())
                .or_insert(HashSet::new())
                .insert(term2.to_string());
            
            network_map
                .entry(term2.to_string())
                .or_insert(HashSet::new())
                .insert(term1.to_string());
        }

        network_map
    };

    //println!("{:?}", network_map);

    println!("Part 1 - {:?}", part1(&network_map));

    println!("Part 2 - {:?}", part2(&network_map));
}
