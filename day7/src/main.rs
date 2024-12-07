use core::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::iter::zip;

const INPUT: &str = "input.txt";
const USE_CONCAT: bool = true;

struct Equation {
    target: isize,
    terms: Vec<isize>,
}

impl fmt::Debug for Equation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {:?}", self.target, self.terms)   
    }
}

impl Equation {
    fn eval(&self) -> bool {
        Self::recursive_eval(self.terms[0], &self.terms[1..], self.target)
    }

    fn recursive_eval(acc: isize, terms: &[isize], target: isize) -> bool {
        if terms.len() <= 0 {
            return acc == target;
        }

        if acc > target {
            return false;
        }

        let mut result: bool;

        result = Self::recursive_eval(acc * terms[0], &terms[1..], target);
        
        if result {
            return result;
        }

        result = Self::recursive_eval(acc + terms[0], &terms[1..], target);

        if result {
            return result;
        }

        if USE_CONCAT {
        let concat_val: String = String::new() + acc.to_string().as_str() + terms[0].to_string().as_str();
        let concat_val: isize = concat_val.parse().expect("Failed to concat numbers");

        result = Self::recursive_eval(concat_val, &terms[1..], target);
        }

        result
    }
}

fn main() {
    let equations: Vec<Equation> = {
        let mut file: File = File::open(INPUT).expect("Failed to open INPUT file");
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read contents of INPUT file");

        let mut equations: Vec<Equation> = Vec::new();
        let lines: Vec<&str> = contents.split("\n").filter(|l| l.len() > 0).collect();

        for line in lines {
            let (target, terms) = line.split_once(":").expect("No ':' to split on");
            let target: isize = target.parse().expect("Could not parse target");

            let terms: Vec<isize> = terms
                .split_whitespace()
                .filter(|t| t.len() > 0)
                .map(|t| t.parse::<isize>().expect("Could not parse term"))
                .collect();

            equations.push(Equation {
                target: target,
                terms: terms,
            });
        }

        equations
    };

    let evals = zip(equations.iter(), equations.iter().map(|e| e.eval())).collect::<Vec<_>>();
    let accumulator: isize = evals.iter().filter(|e| e.1 ).map(|e| e.0.target ).sum();

    for (eq, eq_bool) in evals {
        println!{"{:?} - {:?}", eq, eq_bool};
    }

    println!("{:?}", accumulator);
}
