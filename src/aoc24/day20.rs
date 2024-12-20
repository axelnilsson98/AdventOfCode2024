use std::{collections::HashMap, usize};

pub fn part1(lines: Vec<String>) {
    let positions: HashMap<(i32, i32), i32> = init(lines);
    let cheat_dirs: Vec<(i32, i32)> = vec![
        (2, 0),
        (0, 2),
        (-2, 0),
        (0, -2),
    ];
    let mut cheats: Vec<i32> = Vec::new();

    for (pos, count) in positions.iter() {
        for (dir_i, dir_j) in &cheat_dirs {
            if positions.contains_key(&(pos.0 + dir_i, pos.1 + dir_j)) {
                let cheat_pos = (
                    (pos.0 as i32 + dir_i),
                    (pos.1 as i32 + dir_j),
                );
                let tmp = *&positions.get(&cheat_pos).unwrap();
                if ((tmp - 2) - *count) > 99 {
                    cheats.push(tmp - count - 2);
                }
            }
        }
    }
    println!("{}", cheats.len());
}

fn print_map(map: &Vec<Vec<i32>>) {
    println!();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == -1 {
                print!("{}", " # ");
            } else {
                print!("{:02} ", map[i][j]);
            }
        }
        println!();
    }
    println!();
}

fn valid(map: &Vec<Vec<i32>>, i: i32, j: i32) -> bool {
    if i >= 0 && (i as usize) < map.len() && j >= 0 && (j as usize) < map[i as usize].len() {
        return map[i as usize][j as usize] != -1;
    } else {
        return false;
    }
}

pub fn part2(lines: Vec<String>) {
    let positions = init(lines);
    let mut cheats: Vec<i32> = Vec::new();
    let mut i = 0;
    for (pos, count) in positions.iter() {
        if i % 100 == 0{
            println!("{}", i);
        }
        let cheat_pos: Vec<(i32, i32)> = positions
            .iter()
            .filter(|(c_pos, c_count)| {
                distance(**c_pos, *pos) <= 20
                    && *c_count > count
            }).map(|(c_pos, _)| *c_pos)
            .collect();
        for (c_i, c_j) in cheat_pos {
            let tmp = *positions.get(&(c_i, c_j)).unwrap();
            let d = distance((c_i, c_j), *pos);
            if (tmp - count - d) > 99 {
                cheats.push(tmp - count - d);
            }
        }
        i += 1;
    }
    println!("{}", cheats.len());
}


fn distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()

}

fn init(lines: Vec<String>) -> HashMap<(i32, i32), i32>{
    let mut map: Vec<Vec<i32>> = Vec::new();
    let mut start_pos: (i32, i32) = (0, 0);
    let mut end_pos: (i32, i32) = (0, 0);

    for (i, line) in lines.iter().enumerate() {
        map.push(Vec::new());
        for (j, char) in line.chars().enumerate() {
            if char == '#' {
                map[i].push(-1);
            } else {
                if char == 'S' {
                    start_pos = (i as i32, j as i32);
                } else if char == 'E' {
                    end_pos = (i as i32, j as i32);
                }
                map[i].push(0);
            }
        }
    }
    let mut current_pos: (i32, i32) = start_pos;
    let mut positions: HashMap<(i32, i32), i32> = HashMap::new();

    positions.insert(start_pos, 0);
    let mut counter = 0;
    let dirs: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    'outer: while current_pos != end_pos {
        for (dir_i, dir_j) in &dirs {
            if valid(
                &map,
                current_pos.0 as i32 + dir_i,
                current_pos.1 as i32 + dir_j,
            ) {
                let new_pos = (
                    (current_pos.0 as i32 + dir_i),
                    (current_pos.1 as i32 + dir_j),
                );
                if !positions.contains_key(&new_pos) {
                    counter += 1;
                    positions.insert(new_pos, counter);
                    current_pos = new_pos;
                    continue 'outer;
                }
            }
        }
    }
    positions
}