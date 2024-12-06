use core::panic;
use std::{collections::HashSet, usize};

pub fn part1(lines: Vec<String>) {
    let mut pos: (usize, usize) = (0, 0);
    let mut dir: (i32, i32) = (-1, 0);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let map = lines.iter().enumerate().map(|(i, line)| {
        line.chars().enumerate().map(|(j, char)| {
            if char == '^' {pos = (i,j)}
            char != '#'
        }).collect()
    }).collect();

    while in_bounds(&map, pos) {
        visited.insert(pos);
        if !value(&map, pos, dir){
            dir = turn_right(dir);
        } else {

            pos = (cord(pos.0, dir.0), cord(pos.1, dir.1))
        }
    }
    println!("{}", visited.len());
}

fn in_bounds(map: &Vec<Vec<bool>>, pos: (usize, usize)) -> bool {
    !(pos.0 >= (map.len()) || pos.1 >= (map[0].len()))
}

fn value(map: &Vec<Vec<bool>>, pos: (usize, usize), dir: (i32, i32)) -> bool{
    match map.get(cord(pos.0, dir.0)).and_then(|row| row.get(cord(pos.1, dir.1))){
        Some(value) => return value.clone(),
        None => return true
    }
}

fn cord(p: usize, d: i32) -> usize {
    (p as i32 + d) as usize
}

fn turn_right(dir: (i32, i32)) -> (i32, i32) {
    match dir {
        (-1, 0) => return (0, 1),
        (0, 1) => return (1, 0),
        (1, 0) => return (0, -1),
        (0, -1) => return (-1, 0),
        _ => panic!("wrong turn"),
    }
}
pub fn part2(lines: Vec<String>) {
    let mut pos: (usize, usize) = (0, 0);
    let mut dir: (i32, i32) = (-1, 0);

    let map: Vec<Vec<bool>> = lines.iter().enumerate().map(|(i, line)| {
        line.chars().enumerate().map(|(j, char)| {
            if char == '^' {pos = (i,j)}
            char != '#'
        }).collect()
    }).collect();

    let mut sum = 0;
    for (i, line) in lines.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '^' || char == '#' {
                continue;
            }
            let mut tmp = map.clone();
            tmp[i][j] = false;
            if creates_loop(&tmp, pos, dir) {
                sum += 1;
            }
        }
        println!("{}", i);
    }
    println!("{}", sum);
}

fn creates_loop(map: &Vec<Vec<bool>>, start_pos: (usize, usize), start_dir: (i32, i32)) -> bool {
    let mut visited: HashSet<((usize, usize), (i32, i32))> = HashSet::new();
    let mut dir: (i32, i32) = start_dir;
    let mut pos: (usize, usize) = start_pos;

    while in_bounds(&map, pos) && !visited.contains(&(pos, dir)) {
        visited.insert((pos, dir));
        // println!("{:?}", pos);
        if !value(&map, pos, dir) {
            dir = turn_right(dir);
        } else {
            pos = (cord(pos.0, dir.0), cord(pos.1, dir.1))
        }
    }
    let tmp = in_bounds(&map, pos);
    // println!("{}", tmp);
    tmp
}
