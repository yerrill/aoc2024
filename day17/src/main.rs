use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::usize;

const INPUT: &str = "input.txt";

#[derive(Debug)]
struct Computer {
    program: Vec<u8>,
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
}

impl Computer {
    fn exec(&self) -> Vec<u8> {
        let mut a = self.reg_a;
        let mut b = self.reg_b;
        let mut c = self.reg_c;
        let mut ip = 0;
        let mut output: Vec<u8> = Vec::new();
        let mut temp_output: Vec<usize>;

        while ip < self.program.len() - 1 {
            (a, b, c, ip, temp_output) =
                Computer::step((a, b, c), ip, self.program[ip], self.program[ip + 1]);

            output.extend(
                temp_output
                    .iter()
                    .map(|v| u8::try_from(*v).unwrap())
                    .collect::<Vec<_>>(),
            );
        }

        output
    }

    fn find_a(&self) -> Option<usize> {
        fn dfs(find: &[u8], magic: usize) -> Option<usize> {
            if find.len() <= 0 {
                return Some(magic);
            }

            let found: Option<usize> = None;

            for test in 0..8 {
                let test_val = (magic << 3) + test;
                let (out_digit, _) = next_val(test_val);

                if out_digit == find[0] {
                    if let Some(m) = dfs(&find[1..], test_val) {
                        return Some(m);
                    }
                }
            }

            found
        }

        let reverse_prog = self.program.iter().rev().map(|v| *v).collect::<Vec<_>>();

        dfs(&reverse_prog[..], 0)
    }

    fn write_instructions(&self) -> String {
        let mut out: String = String::new();
        let mut ip = 0;

        fn write_combo(operand: u8) -> String {
            String::new()
                + (match operand {
                    0 | 1 | 2 | 3 => format!("({})", operand.to_string()),
                    4 => "A".to_string(),
                    5 => "B".to_string(),
                    6 => "C".to_string(),
                    7 => panic!("7 combo operator"),
                    _ => panic!("Unknown combo"),
                })
                .as_str()
        }

        while ip < self.program.len() - 1 {
            let operator = self.program[ip];
            let operand = self.program[ip + 1];

            out += (match self.program[ip] {
                0 => format!("0 adv {}", write_combo(operand)),
                1 => format!("1 bxl {}", operand),
                2 => format!("2 bst {}", write_combo(operand)),
                3 => format!("3 jnz {}", operand),
                4 => format!("4 bxc _"),
                5 => format!("5 out {}", write_combo(operand)),
                6 => format!("6 bdv {}", write_combo(operand)),
                7 => format!("7 cdv {}", write_combo(operand)),
                _ => panic!("Unknown operator {:?}", operator),
            })
            .as_str();

            ip += 2;

            out += "\n";
        }

        out
    }

    fn step(
        (reg_a, reg_b, reg_c): (usize, usize, usize),
        ip: usize,
        operator: u8,
        operand: u8,
    ) -> (usize, usize, usize, usize, Vec<usize>) {
        let mut a = reg_a;
        let mut b = reg_b;
        let mut c = reg_c;
        let mut ip: usize = ip;
        let mut output: Vec<usize> = Vec::new();

        let mut jumped = false;

        match operator {
            0 => {
                (a, b, c) = Computer::adv(operand, (a, b, c));
            }
            1 => {
                (a, b, c) = Computer::bxl(operand, (a, b, c));
            }
            2 => {
                (a, b, c) = Computer::bst(operand, (a, b, c));
            }
            3 => {
                if let Some(new_ip) = Computer::jnz(operand, (a, b, c), ip) {
                    ip = new_ip;
                    jumped = true;
                }
            }
            4 => {
                (a, b, c) = Computer::bxc(operand, (a, b, c));
            }
            5 => {
                output.push(Computer::out(operand, (a, b, c)));
            }
            6 => {
                (a, b, c) = Computer::bdv(operand, (a, b, c));
            }
            7 => {
                (a, b, c) = Computer::cdv(operand, (a, b, c));
            }
            _ => panic!("Unknown operator {:?}", operator),
        }

        if !jumped {
            ip += 2;
        }

        (a, b, c, ip, output)
    }

    fn combo(operand: u8, (a, b, c): (usize, usize, usize)) -> usize {
        match operand {
            0 | 1 | 2 | 3 => operand as usize,
            4 => a,
            5 => b,
            6 => c,
            7 => panic!("7 combo operator {:?} {:?}", operand, (a, b, c)),
            _ => panic!("Unknown combo {:?}", operand),
        }
    }

    fn adv(operand: u8, (a, b, c): (usize, usize, usize)) -> (usize, usize, usize) {
        let combo = Computer::combo(operand, (a, b, c));
        (a / 2_usize.pow(combo as u32), b, c)
    }

    fn bxl(operand: u8, (a, b, c): (usize, usize, usize)) -> (usize, usize, usize) {
        (a, b ^ operand as usize, c)
    }

    fn bst(operand: u8, (a, b, c): (usize, usize, usize)) -> (usize, usize, usize) {
        let combo = Computer::combo(operand, (a, b, c));
        (a, combo % 8, c)
    }

    fn jnz(operand: u8, (a, _, _): (usize, usize, usize), _: usize) -> Option<usize> {
        if a != 0 {
            Some(operand as usize)
        } else {
            None
        }
    }

    fn bxc(_: u8, (a, b, c): (usize, usize, usize)) -> (usize, usize, usize) {
        (a, b ^ c, c)
    }

    fn out(operand: u8, (a, b, c): (usize, usize, usize)) -> usize {
        let combo = Computer::combo(operand, (a, b, c));
        combo % 8
    }

    fn bdv(operand: u8, (a, b, c): (usize, usize, usize)) -> (usize, usize, usize) {
        let combo = Computer::combo(operand, (a, b, c));
        (a, a / 2_usize.pow(combo as u32), c)
    }

    fn cdv(operand: u8, (a, b, c): (usize, usize, usize)) -> (usize, usize, usize) {
        let combo = Computer::combo(operand, (a, b, c));
        (a, b, a / 2_usize.pow(combo as u32))
    }
}

fn parse_input(contents: &String) -> Computer {
    let re = Regex::new(r"Register A: ([0-9]*)\nRegister B: ([0-9]*)\nRegister C: ([0-9]*)\n\nProgram: ((?:[0-9]*,)+[0-9]*)").unwrap();
    let hay = contents.as_str();

    let caps = re.captures(hay).unwrap();

    Computer {
        program: {
            let mut v: Vec<u8> = Vec::new();

            for s in caps[4].split(",") {
                v.push(s.parse().unwrap())
            }

            v
        },
        reg_a: *(&caps[1].parse().unwrap()),
        reg_b: *(&caps[2].parse().unwrap()),
        reg_c: *(&caps[3].parse().unwrap()),
    }
}

// Based on reverse engineering my puzzle
fn next_val(a: usize) -> (u8, usize) {
    let mut a: usize = a;
    let mut b: usize;
    let c: usize;

    b = a % 8;
    b = b ^ 7;
    c = a / 2_usize.pow(b as u32);
    b = b ^ 7;
    a = a / 2_usize.pow(3_u32);
    b = b ^ c;
    ((b % 8) as u8, a)
}

fn write_a_val(a: usize) {
    let mut val = next_val(a);
    println!("{:?}", val);

    while val.1 > 0 {
        val = next_val(val.1);
        println!("{:?}", val);
    }
}

fn main() {
    let comp: Computer = {
        let mut file: File = File::open(INPUT).expect("Failed to open INPUT file");
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read contents of INPUT file");

        parse_input(&contents)
    };

    println!("{:?}", comp.exec());
    println!("\n{}\n", comp.write_instructions());

    let a_val = comp.find_a();
    println!("{:?}", a_val);
    if let Some(v) = a_val {
        write_a_val(v);
    }
}
