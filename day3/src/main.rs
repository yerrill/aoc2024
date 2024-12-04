use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

const INPUT: &str = "input.txt";
const LEADER: &str = "mul(";
const DIVIDER: &str = ",";
const END: &str = ")";
const DO: &str = "do()";
const DONT: &str = "don't()";

type SliceStart = usize;
type SliceEnd = usize;

struct Parser {
    input: String,
}

impl std::ops::Index<std::ops::Range<usize>> for Parser {
    type Output = str;

    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        &self.input[index.start..index.end]
    }
}

impl Parser {
    fn len(&self) -> usize {
        self.input.len()
    }

    fn seek_substring(&self, substr: &str, start: usize) -> Option<(SliceStart, SliceEnd)> {
        let substring_len: usize = substr.len();
        let mut window_start: usize = start;
    
        while window_start + substring_len <= self.len() {
    
            if self[window_start..window_start+substring_len] == *substr {
                return Some((window_start, window_start + substring_len));
            }
    
            window_start += 1;
        }
    
        None
    }

    fn seek_substring_backward(&self, substr: &str, start: usize) -> Option<(SliceStart, SliceEnd)> {
        let substring_len: usize = substr.len();
        let mut window_start: usize = start;
    
        loop {
    
            if self[window_start..window_start+substring_len] == *substr {
                return Some((window_start, window_start + substring_len));
            }
    
            if window_start <= 0 { break; } else { window_start -= 1; }
        }
    
        None
    }

    fn scan_int(&self, start: usize) -> Option<(usize, SliceEnd)> {
        let mut slice_size: usize = 0;
        let mut parsed_int: usize = 0;
    
        while start + slice_size < self.len() {
            let next_slice: &str = &self[start..start+slice_size+1];
            let parsed = next_slice.parse::<usize>();
    
            if let Ok(val) = parsed {
                parsed_int = val;
                slice_size += 1;
            } else {
                break;
            }
        }
    
        if slice_size > 0 {
            Some((parsed_int, slice_size + start))
        } else {
            None
        }
    }

    fn expect_substring(&self, substr: &str, start: usize) -> Option<SliceEnd> {
        let slice_end = start + substr.len();
    
        if &self[start..slice_end] == substr {
            Some(slice_end)
        } else {
            None
        }
    }
}

fn find_mult(input: &Parser) {
    // Pretty much all this to match `mul\([0-9]*,[0-9]*\)`

    fn try_construct(input: &Parser, start: usize) -> Option<(usize, usize, SliceEnd)> {
        
        let (first_num, first_num_index) = input.scan_int(start)?;

        let seperator_index = input.expect_substring(DIVIDER, first_num_index)?;

        let (second_num, second_num_index) = input.scan_int(seperator_index)?;

        let end_index = input.expect_substring(END, second_num_index)?;

        Some((first_num, second_num, end_index))
    }

    fn check_qualified(input: &Parser, start: usize) -> bool {
        let last_do = input.seek_substring_backward(DO, start);
        let last_dont = input.seek_substring_backward(DONT, start);

        let result = match (last_do, last_dont) {
            (Some((_, do_index)), Some((_, dont_index))) => do_index > dont_index,
            (None, Some(_)) => false,
            (Some(_), None) => true,
            (None, None) => true
        };

        result
    }

    let mut last_end = 0;

    // Standard sets counting all mul's
    let mut accumulator_standard = 0;
    let mut sets_found_standard = 0;

    // mul's that are qualified by a do/don't
    let mut accumulator_qualified = 0;
    let mut sets_found_qualified = 0;

    while let Some((a, b)) = input.seek_substring(LEADER, last_end) {
        if let Some((first, second, end_index)) = try_construct(input, b) {
            sets_found_standard += 1;
            accumulator_standard += first * second;

            if check_qualified(input, a) {
                sets_found_qualified += 1;
                accumulator_qualified += first * second;
            }

            last_end = end_index;

            //println!("mul({:?},{:?}) - {:?}", first, second, last_end);
        } else {
            last_end = b;
        }
    }

    println!("Parser Part 1 (Standard) - Accumulator: {:?}, Sets Found: {:?}", accumulator_standard, sets_found_standard);
    println!("Parser Part 2 (Qualified) - Accumulator: {:?}, Sets Found: {:?}", accumulator_qualified, sets_found_qualified);

}


fn regex_method(input: &Parser) {
    let re: Regex = Regex::new(r"(do\(\))|(don't\(\))|(mul\([0-9]*,[0-9]*\))").unwrap();
    let hay: &str = input.input.as_str();

    let mut accumulator = 0;
    let mut sets = 0;
    let mut enable = true;

    for (_, [cap]) in re.captures_iter(hay).map(|c| c.extract()) {
        if cap == DO {
            enable = true;
        } else if cap == DONT {
            enable = false;
        } else if enable {
            accumulator += cap[4..cap.len()-1].split(DIVIDER).map(|e| e.parse::<usize>().unwrap()).reduce(|acc, e| acc * e).unwrap();
            sets += 1
        }
    }

    println!("Regex Part 2 (Qualified) - Accumulator: {:?}, Sets Found: {:?}", accumulator, sets);
}

fn main() {
    let mut file: File = File::open(INPUT).expect("Failed to open file");
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read contents");

    let parser: Parser = Parser{input: contents};

    find_mult(&parser);
    regex_method(&parser);

}
