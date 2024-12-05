use core::panic;
use std::{
    collections::{HashMap, HashSet},
    ops::Index,
    usize,
};

pub fn part1(lines: Vec<String>) {
    if let Some(index) = lines.iter().position(|s| s == "") {
        let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
        let rules_lines = &lines[0..index]; 
        for (i, rule) in rules_lines.iter().enumerate() {
            let nums: Vec<i32> = rule
                .split('|')
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            rules.entry(nums[0]).or_insert_with(Vec::new).push(nums[1]);
        }

        let mut sum = 0;
        let prints = &lines[index+1..];
  
        for (i, print) in prints.iter().enumerate() {
            let pages: Vec<i32> = print
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            let mut correct = true;
            for (j, page) in pages.iter().enumerate().rev() {
                if find_incorrect(&pages, &rules) != (usize::max_value(), usize::max_value()) {
                    correct = false;
                    break;
                }
            }
            if correct {
                sum += pages[pages.len() / 2];
            }
        }
        println!("{}", sum);
    }
}

pub fn part2(lines: Vec<String>) {
    if let Some(index) = lines.iter().position(|s| s == "") {
        let mut incorrect: Vec<Vec<i32>> = Vec::new();
        let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
        for i in 0..index {
            let nums: Vec<i32> = lines[i]
                .split('|')
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            rules.entry(nums[0]).or_insert_with(Vec::new).push(nums[1]);
        }

        for i in (index + 1)..lines.len() {
            let pages: Vec<i32> = lines[i]
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            if find_incorrect(&pages, &rules) != (usize::max_value(), usize::max_value()) {
                incorrect.push(pages);
            }
        }

        let sum = incorrect
            .iter()
            .map(|xs| fix(xs, &rules))
            .map(|xs| xs[xs.len() / 2])
            .sum::<i32>();
        println!("{}", sum);
    }
}

fn fix(xs: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut res: Vec<i32> = xs.clone();
    let (mut index, mut index2) = find_incorrect(&res, rules);
    while index != usize::max_value() {
        let temp: i32 = res[index as usize];
        res[index as usize] = res[(index2) as usize];
        res[(index2) as usize] = temp;
        (index, index2) = find_incorrect(&res, rules);
    }
    res
}

fn find_incorrect(xs: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> (usize, usize) {
    for (j, page) in xs.iter().enumerate().rev() {
        if let Some(rule) = rules.get(&page) {
            let set1: HashSet<&i32> = rule.iter().collect();
            let set2: HashSet<&i32> = xs[..j].iter().collect();
            let overlap: Vec<&i32> = set1.intersection(&set2).cloned().collect();
            if !overlap.is_empty() {
                if let Some(err) = xs.iter().position(|x| x == overlap[0]) {
                    return (j, err);
                }
            }
        }
    }
    return (usize::max_value(), usize::max_value());
}
