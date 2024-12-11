use core::num;
use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
    usize,
};

pub fn part1(lines: Vec<String>) {
    let num_iter = 75;
    let mut current_numbers: Vec<i64> = lines[0].split(" ").map(|x| x.parse().unwrap()).collect();
    let mut new_numbers: Vec<i64> = Vec::new();
    // println!("{:?}", current_numbers);

    for i in 0..num_iter {
        println!("{}", i);
        new_numbers = Vec::new();
        for num in &current_numbers {
            if *num == 0 {
                new_numbers.push(1);
                continue;
            }
            let digits = num_digits(*num);
            if digits % 2 == 0 {
                let pow = i64::pow(10, (digits / 2) as u32);
                let val1 = num / pow;
                let val2 = num - val1 * pow;
                new_numbers.push(val1);
                new_numbers.push(val2);
                continue;
            }
            new_numbers.push(num * 2024);
        }
        current_numbers = new_numbers;
    }

    println!("{}", current_numbers.len());
}

pub fn part2(lines: Vec<String>) {
    let num_iter = 25;
    let mut current_numbers: HashMap<i64, usize> = HashMap::new();
    lines[0].split(" ").for_each(|x| {
        current_numbers.insert(x.parse().unwrap(), 1);
    });

    let mut new_numbers: HashMap<i64, usize> = HashMap::new();

    for i in 0..num_iter {
        new_numbers = HashMap::new();
        for (num, nbrof) in &current_numbers {
            if *num == 0 {
                let new_val = get_or_else(&new_numbers, 1);
                new_numbers.insert(1, new_val+nbrof);
                continue;
            }
            let digits = num_digits(*num);
            if digits % 2 == 0 {
                let pow = i64::pow(10, (digits / 2) as u32);
                let val1 = num / pow;
                let val2 = num - val1 * pow;
                let new_val = get_or_else(&new_numbers, val1);
                new_numbers.insert(val1, new_val+nbrof);
                let new_val = get_or_else(&new_numbers, val2);
                new_numbers.insert(val2, new_val+nbrof);
                continue;
            }
            let new_val = get_or_else(&new_numbers, num*2024);
            new_numbers.insert(num*2024, new_val+nbrof);
    }
        current_numbers = new_numbers;
    }
    let mut sum = 0;
    current_numbers.iter().for_each(|(_, numi)| sum += numi);
    println!("{}", sum);
}

fn get_or_else(hm: &HashMap<i64, usize>, k: i64) -> usize{
    let value = hm.get(&k);
    match value {
        Some(v) => {
            return *v;
        }
        None => return 0,
    };
}

fn num_digits(mut n: i64) -> i64 {
    if n == 0 {
        return 1;
    }
    let mut digits = 0;
    while n != 0 {
        n /= 10;
        digits += 1;
    }
    digits
}
