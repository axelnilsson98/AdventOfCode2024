use std::{
    char,
    collections::{hash_set, HashSet},
    hash::Hash,
    ptr::null,
    usize,
};

use itertools::Combinations;

pub fn part1(lines: Vec<String>) {
    let map: Vec<Vec<usize>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let mut combinations: HashSet<(usize, usize, usize, usize)> = HashSet::new();
    for (i, row) in map.iter().enumerate(){
        for (j, height) in row.iter().enumerate(){
            if *height == 0 {
                combinations.extend(find_trail(&map, i ,j));
            }
        }
    }
    println!("{}", combinations.len());
}

pub fn part2(lines: Vec<String>) {
    let mut combinations: Vec<(usize, usize, usize, usize)> = Vec::new();
    let map: Vec<Vec<usize>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    for (i, row) in map.iter().enumerate() {
        for (j, height) in row.iter().enumerate() {
            if *height == 0 {
                combinations.extend(find_trail(&map, i, j));
            }
        }
    }
    println!("{}", combinations.len());
}

fn find_trail(map: &Vec<Vec<usize>>, i: usize, j: usize) -> Vec<(usize, usize, usize, usize)> {
    let res = find_trail_helper(map, i, j);
    let tmp = res
        .iter()
        .map(|(i_dest, j_dest)| (i, j, *i_dest, *j_dest))
        .collect();
    tmp
}

fn find_trail_helper(map: &Vec<Vec<usize>>, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    let num = map[i][j];
    if num == 9 {
        res.push((i, j));
        return res;
    } else {
        for (i_new, j_new) in get_dirs(&map, i, j) {
            // println!("new cord to search {}, {}", i_new, j_new);
            if map[i_new][j_new] == num + 1 {
                res.extend(find_trail_helper(&map, i_new, j_new));
            }
        }
        return res;
    }
}

fn get_dirs(map: &Vec<Vec<usize>>, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();

    if i > 0 {
        res.push((i - 1, j));
    }
    if j > 0 {
        res.push((i, j - 1));
    }
    if j < map[0].len() - 1 {
        res.push((i, j + 1));
    }
    if i < map.len() - 1 {
        res.push((i + 1, j))
    }
    res
}
