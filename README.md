# aoc2024

Advent of Code 2024

## File input I seem to use every time

```rust
use std::fs::File;
use std::io::prelude::*;

const INPUT: &str = "input.txt";

fn main() {
    let result = {
        let mut file: File = File::open(INPUT).expect("Failed to open INPUT file");
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read contents of INPUT file");
    };
}
```
