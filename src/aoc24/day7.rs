use itertools::Itertools;

pub fn part1(lines: Vec<String>) {
    let mut linenbr = 0;
    let mut sum: i64 = 0;
    for line in lines {
        let first_split: Vec<&str> = line.split(":").collect();

        let value: i64 = first_split[0].parse().unwrap();

        let operands: Vec<i64> = first_split[1][1..]
            .split(" ")
            .map(|x: &str| x.parse().unwrap())
            .collect();
        if add(&operands[1..], operands[0], value) || mul(&operands[1..], operands[0], value) || concat(&operands[1..], operands[0], value) {
            sum += value;
        }
        linenbr += 1;
    }
    println!("{}", sum);
}

pub fn part2(lines: Vec<String>) {}

fn add(vec: &[i64], current: i64, target: i64) -> bool {
    let res = current + vec[0];
    if res > target {
        return false;
    } else if vec.len() == 1 {
        if res == target {
        }
        return res == target;
    }
    let passon: &[i64] = &vec[1..];
    return add(passon, res, target) || mul(passon, res, target) || concat(passon, res, target);
}

fn mul(vec: &[i64], current: i64, target: i64) -> bool {
    let res = current * vec[0];
    if res > target {

        return false;
    } else if vec.len() == 1 {
        if res == target {
        }
        return res == target;
    }
    let passon: &[i64] = &vec[1..];
    return add(passon, res, target) || mul(passon, res, target) || concat(passon, res, target);
}

fn concat(vec: &[i64], current: i64, target: i64) -> bool {
    let concatenated = format!("{}{}", current, vec[0]);
    let res = concatenated.parse().unwrap();
    if res > target {

        return false;
    } else if vec.len() == 1 {
        if res == target {
        }

        return res == target;
    }
    let passon: &[i64] = &vec[1..];
    return add(passon, res, target) || mul(passon, res, target) || concat(passon, res, target);
}
