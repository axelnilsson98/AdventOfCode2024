use std::collections::{HashMap, HashSet, VecDeque};

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use itertools::Itertools;

pub fn part1(lines: Vec<String>) {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut start_pos: (usize, usize) = (0, 0);

    for (i, line) in lines.iter().enumerate() {
        if line.contains("S") {
            start_pos = (i, line.find("S").unwrap());
        }
        map.push(line.chars().collect());
    }

    print_map(&map);
    let mut cost = 0;
    let mut s = 0;
    //maybe needs to keep track of what the cost was
    let mut visited: HashMap<(usize, usize, i32, i32), usize> = HashMap::new();
    let mut to_handle: VecDeque<(usize, usize, usize, i32, i32, i32)> = VecDeque::new();
    to_handle.push_front((start_pos.0, start_pos.1, 0, 0, 1, 0));
    while !to_handle.is_empty() {
        to_handle = to_handle
            .iter()
            .sorted_by(|a, b| Ord::cmp(&a.2, &b.2))
            .map(|t| *t)
            .collect();
        let (i, j, c, dir_i, dir_j, steps) = to_handle.pop_front().unwrap();
        if let Some(stored_c) = visited.get(&(i, j, dir_i, dir_j)) {
            if *stored_c <= c {
                continue;
            } else {
                visited.insert((i, j, dir_i, dir_j), c);
            }
        }
        visited.insert((i, j, dir_i, dir_j), c);


        // Check for the end condition
        if map[i][j] == 'E' {
            cost = c;
            s = steps;
            println!("found route");
            // break;
        }

        //process movement
        for &(step_cost, rotation) in &[
            (1001, rotate_clockwise(dir_i, dir_j)),
            (1001, rotate_counter_clockwise(dir_i, dir_j)),
            (1, (dir_i, dir_j)),
        ] {
            let next = step((i, j), rotation);
            if map[next.0][next.1] == '#' {
                continue;
            }
            if !visited.contains_key(&(next.0, next.1, rotation.0, rotation.1)) {
                to_handle.push_back((
                    next.0,
                    next.1,
                    c + step_cost,
                    rotation.0,
                    rotation.1,
                    steps + 1,
                ));
            }
        }
    }
    println!("cost: {}, steps: {}", cost, s);
}

fn print_map(map: &Vec<Vec<char>>) {
    println!();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            print!("{}", map[i][j]);
        }
        println!();
    }
    println!();
}

fn rotate_clockwise(i: i32, j: i32) -> (i32, i32) {
    (j, -i)
}
fn rotate_counter_clockwise(i: i32, j: i32) -> (i32, i32) {
    (-j, i)
}
fn step(pos: (usize, usize), dir: (i32, i32)) -> (usize, usize) {
    (
        (pos.0 as i32 + dir.0) as usize,
        (pos.1 as i32 + dir.1) as usize,
    )
}
pub fn part2(lines: Vec<String>) {
    let mut actual_cost = 100000000000000000;
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut start_pos: (usize, usize) = (0, 0);

    for (i, line) in lines.iter().enumerate() {
        if line.contains("S") {
            start_pos = (i, line.find("S").unwrap());
        }
        map.push(line.chars().collect());
    }
    print_map(&map);
    let mut _cost = 0;
    let mut _s = 0;
    //maybe needs to keep track of what the cost was
    let mut best_seats: HashSet<(usize, usize)> = HashSet::new();
    let mut visited: HashMap<(usize, usize, i32, i32), usize> = HashMap::new();
    let mut to_handle = BinaryHeap::new();
    let binding = (start_pos.0, start_pos.1, 0, 0, 1, 0, Vec::new());
    to_handle.push(Reverse((binding.3, binding)));
    while let Some(Reverse((_priority, (i, j, c, dir_i, dir_j, steps, path)))) = to_handle.pop() {
        if let Some(stored_c) = visited.get(&(i, j, dir_i, dir_j)) {
            if *stored_c < c {
                continue;
            }
        }
        visited.insert((i, j, dir_i, dir_j), c);

        // Check for the end condition
        if map[i][j] == 'E' {
            let mut tmp_path = path.clone();
            tmp_path.push((i, j)); // Add 'E' to the path
            if c <= actual_cost {
                actual_cost = c;
                best_seats.extend(tmp_path);
            }
            println!("found route");
            // break;
        }

        // Process movements
        for &(rotation_cost, rotation) in &[
            (1001, rotate_clockwise(dir_i, dir_j)),
            (1001, rotate_counter_clockwise(dir_i, dir_j)),
            (1, (dir_i, dir_j)),
        ] {
            let next = step((i, j), rotation);
            if map[next.0][next.1] == '#' {
                continue;
            }
            let mut tmp_path = path.clone();
            tmp_path.push((next.0, next.1));
            if !visited.contains_key(&(next.0, next.1, rotation.0, rotation.1)) {
                to_handle.push(Reverse((
                    (c + rotation_cost) as i32,
                    (
                        next.0,
                        next.1,
                        c + rotation_cost,
                        rotation.0,
                        rotation.1,
                        steps + 1,
                        tmp_path,
                    ),
                )));
            }
        }
    }

    // Mark the path on the map
    for (i, j) in &best_seats {
        map[*i][*j] = '0';
    }
    print_map(&map);
    //+1 for offset startpos
    println!("Path length: {}", best_seats.len() + 1);
}
