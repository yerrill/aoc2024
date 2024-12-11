use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

const INPUT: &str = "input.txt";
const BLINKS: usize = 75;

struct Cache {
    map: HashMap<usize, [usize; BLINKS]>,
    hits: usize,
    misses: usize,
}

impl std::fmt::Debug for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out: String = String::new();

        for (key, val) in self.map.iter() {
            out += format!("{:?}: {:?}", key, val).as_str();
            out += "\n";
        }

        f.write_str(out.as_str())
    }
}

impl Cache {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            hits: 0,
            misses: 0
        }
    }

    fn get(&mut self, index: usize, iteration: usize) -> Option<usize> {
        assert!(iteration < BLINKS);

        if let Some(arr) = self.map.get(&index) {
            if arr[iteration] > 0 {
                self.hits += 1;
                Some(arr[iteration])
            } else {
                self.misses += 1;
                None
            }
        } else {
            self.misses += 1;
            None
        }
    }

    fn insert(&mut self, index: usize, iteration: usize, value: usize) {
        assert!(iteration < BLINKS);

        if let Some(arr) = self.map.get_mut(&index) {
            arr[iteration] = value;
        } else {
            let mut arr: [usize; BLINKS] = [0; BLINKS];
            arr[iteration] = value;
            self.map.insert(index, arr);
        }
    }
}

fn stone_counter(cache: &mut Cache, stone_num: usize, iteration: usize) -> usize {
    if iteration >= BLINKS {
        //print!("{stone_num:?} "); // Uncomment to see final stone order
        return 1;
    }

    if let Some(v) = cache.get(stone_num, iteration) {
        return v;
    }

    let stone_string: String = stone_num.to_string();
    let stone_str: &str = stone_string.as_str();
    let midpoint: usize = stone_str.len() / 2;

    let result;

    if stone_num == 0 {
        result = stone_counter(cache, 1, iteration + 1);
    } else if stone_str.len() % 2 == 0 {
        let left: usize = stone_str[0..midpoint].parse().unwrap();
        let right: usize = stone_str[midpoint..stone_str.len()].parse().unwrap();
        result =
            stone_counter(cache, left, iteration + 1) + stone_counter(cache, right, iteration + 1);
    } else {
        result = stone_counter(cache, stone_num * 2024, iteration + 1);
    }

    cache.insert(stone_num, iteration, result);

    result
}

fn main() {
    let starting_values: Vec<usize> = {
        let mut file: File = File::open(INPUT).expect("Failed to open INPUT file");
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read contents of INPUT file");

        let mut parsed_nums: Vec<usize> = Vec::new();

        for num in contents
            .chars()
            .filter(|c| c.is_numeric() || *c == ' ')
            .collect::<String>()
            .split(" ")
        {
            parsed_nums.push(num.parse().expect("failed  to parse"));
        }

        parsed_nums
    };

    let mut cache: Cache = Cache::new();
    let mut acc: usize = 0;

    for val in starting_values.iter() {
        acc += stone_counter(&mut cache, *val, 0);
    }

    println!("{:?}", starting_values);
    println!("Cache - Hits: {:?}, Misses: {:?}", cache.hits, cache.misses);
    println!("{:?}", cache);
    println!("Stones: {:?}", acc);
}
