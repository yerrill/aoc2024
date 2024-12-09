use std::fs::File;
use std::io::prelude::*;

const INPUT: &str = "input.txt";

type ID = u16;
type Block = Option<ID>;

fn _part1_rearrange_disk(vec: &mut Vec<Block>) {
    let mut left: usize = 0;
    let mut right: usize = vec.len() - 1;

    while left < right {
        
        match (vec[left], vec[right]) {
            (_, None) => { right -= 1; },
            (Some(_), _) => { left += 1; },
            (None, Some(val)) => {
                vec[left] = Some(val);
                vec[right] = None;

                left += 1;
                right -= 1;
            },
        }
    }
}

fn part2_rearrange_disk(vec: &mut Vec<Block>) {

    fn find_contiguous_space(disk: &Vec<Block>, size: usize, right_max: usize) -> Option<(usize, usize)> {
        let mut left: usize = 0;
        let mut found: Option<(usize, usize)> = None;

        while left < right_max {
            if let None = disk[left] {
                if let Some((l, _)) = found {
                    found = Some((l, left))
                } else {
                    found = Some((left, left))
                }

                if let Some((l, r)) = found {
                    if r - l + 1 >= size { break; }
                }

            } else {
                found = None;
            }

            left += 1;
        }

        if left >= right_max {
            found = None
        }

        found
    }

    fn get_current_block(disk: &Vec<Block>, start: usize) -> (usize, usize) {
        let value = disk[start];
        let mut left = start;
        let mut right = start + 1;

        for (i, v) in disk.iter().enumerate().skip(start) {
            if *v == value {
                right = i;
            } else {
                break;
            }
        }

        for (i, v) in disk.iter().enumerate().rev().skip(disk.len() - start - 1) {
            if *v == value {
                left = i;
            } else {
                break;
            }
        }

        (left, right)
    }

    fn copy_file(disk: &mut Vec<Block>, dest: (usize, usize), source: (usize, usize)) {
        assert_eq!(dest.1 - dest.0 , source.1 - source.0, "{:?} {:?}", dest, source);

        for i in 0..dest.1-dest.0+1 {
            disk[dest.0+i] = disk[source.0+i];
            disk[source.0+i] = None;
        }
    }

    let mut current_block: (usize, usize);
    let mut avail_space: Option<(usize, usize)>;
    let mut i: usize = vec.len() - 1;

    while i > 0 {
        current_block = get_current_block(&vec, i);
        avail_space = find_contiguous_space(&vec, current_block.1 - current_block.0 + 1, current_block.0);

        if let (Some(_), Some(space)) = (vec[i], avail_space) {
            copy_file(vec, space, current_block);
        }

        // Skip to next block
        if current_block.0 <= 0 {
            i = 0;
        } else {
            i = current_block.0 - 1;
        }
    }
}

fn checksum(vec: &Vec<Block>) -> usize {
    let mut acc: usize = 0;

    for (pos, e) in vec.iter().enumerate() {
        if let Some(v) = e {
            acc += pos * usize::from(*v);
        }
    }

    acc
}

fn main() {
    let mut disk_map: Vec<Block> = {
        let mut file: File = File::open(INPUT).expect("Failed to open INPUT file");
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read contents of INPUT file");

        fn push_n_times(vec: &mut Vec<Block>, repeats: ID, value: Block) {
            for _ in 0..repeats {
                vec.push(value);
            }
        }

        let mut disk_map: Vec<Block> = Vec::new();
        let mut file: bool = true;
        let mut file_id: ID = 0;

        for val in contents.chars().filter(|ch| ch.is_numeric()) {
            let num = val.to_string().parse::<ID>().expect("Failed to parse");

            if file {
                push_n_times(&mut disk_map, num, Some(file_id));
                file_id += 1;
            } else {
                push_n_times(&mut disk_map, num, None);
            }

            file = !file;
        }

        disk_map
    };

    //part1_rearrange_disk(&mut disk_map);
    //println!("{:?} {:?}", disk_map, checksum(&disk_map));

    part2_rearrange_disk(&mut disk_map);
    println!("{:?} {:?}", disk_map, checksum(&disk_map));
}
