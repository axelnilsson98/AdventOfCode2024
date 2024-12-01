#![allow(dead_code)]
mod aoc24; 

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()>{

    let file_path: &str = "/home/axel/Documents/games/aoc24/src/day2.txt";
    let file: File = File::open(file_path)?;
    let reader: BufReader<File> = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines()
        .map(|line| line.unwrap()) // Handle Result by unwrapping (use ? if errors should propagate)
        .collect();

    aoc24::day2::part1(lines);
    Ok(())

}
