use std::collections::HashSet;

use itertools::Itertools;

pub fn part1(lines: Vec<String>) {
    let mut linenbr = 0;
    let mut res: i64 = 0;
    for line in lines {
        // println!("line: {}", linenbr);
        linenbr += 1;
        let first_split: Vec<&str> = line.split(":").collect();

        let value: i64 = first_split[0].parse().unwrap();

        let operands: Vec<i64> = first_split[1][1..]
            .split(" ")
            .map(|x: &str| x.parse().unwrap())
            .collect();
        // println!("num of operands: {}", operands.len());
        if add(&operands, 0, value) || mul(&operands, 0, value) || concat(&operands, 0, value) {
            // println!("true");
            res += value;
        }
    }
    println!("{}", res);
}

pub fn part2(lines: Vec<String>) {}

fn add(vec: &[i64], current: i64, target: i64) -> bool {
    let res = current + vec[0];
    if res == target {
        // println!("add returns true");
        return true;
    } else if vec.len() == 1 {
        // println!("reached end, res: {}", res);

        return false;
    }
    let passon: &[i64] = &vec[1..];
    return add(passon, res, target) || mul(passon, res, target) || concat(passon, res, target);
}

fn mul(vec: &[i64], current: i64, target: i64) -> bool {
    let res = current * vec[0];
    if res == target {
        // println!("mul returns true");

        return true;
    } else if vec.len() == 1 || res > target {
        // println!("reached end, res: {}", res);
        return false;
    }
    let passon: &[i64] = &vec[1..];
    return add(passon, res, target) || mul(passon, res, target) || concat(passon, res, target);
}

fn concat(vec: &[i64], current: i64, target: i64) -> bool {
    let concatenated = format!("{}{}", current, vec[0]);

    let res = concatenated.parse().unwrap();
    if res == target {
        // println!("mul returns true");

        return true;
    } else if vec.len() == 1 || res > target {
        // println!("reached end, res: {}", res);
        return false;
    }
    let passon: &[i64] = &vec[1..];
    return add(passon, res, target) || mul(passon, res, target) || concat(passon, res, target);
}
