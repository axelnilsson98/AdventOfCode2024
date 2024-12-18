#![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]
// #![allow(unused_assignments)]
mod aoc24; 

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()>{

    let file_path: &str = "/home/axel/Documents/games/aoc24/src/input/day18.txt";
    let file: File = File::open(file_path)?;
    let reader: BufReader<File> = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines()
        .map(|line| line.unwrap()) // Handle Result by unwrapping (use ? if errors should propagate)
        .collect();

    aoc24::pattern::pattern();
    Ok(())

}
