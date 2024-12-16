use std::io;

static MAX_X: i32 = 101;
static MAX_Y: i32 = 103;
pub fn part1(lines: Vec<String>) {
    // let max_x: i32 = 101;
    let mid_x = MAX_X / 2;
    // let max_y = 103;
    let mid_y = MAX_Y / 2;
    let mut quadrants = vec![0, 0, 0, 0];
    for line in lines {
        let nums: Vec<i32> = line
            .split('=')
            .flat_map(|x| x.split(' '))
            .flat_map(|x| x.split(','))
            .map(|x| x.parse::<i32>())
            .filter(|x: &Result<i32, std::num::ParseIntError>| is_ok(x))
            .map(|x| x.unwrap())
            .collect();

        let new_pos = (
            (nums[0] + nums[2] * 100).rem_euclid(MAX_X),
            (nums[1] + nums[3] * 100).rem_euclid(MAX_Y),
        );
        // println!("{:?}", new_pos);
        if new_pos.0 < mid_x && new_pos.1 < mid_y {
            quadrants[0] += 1
        } else if new_pos.0 < mid_x && new_pos.1 > mid_y {
            quadrants[1] += 1
        } else if new_pos.0 > mid_x && new_pos.1 < mid_y {
            quadrants[2] += 1
        } else if new_pos.0 > mid_x && new_pos.1 > mid_y {
            quadrants[3] += 1
        } else {
            // println!("discarded: {:?}", new_pos);
        }
    }
    println!("{:?}", quadrants);
    let sum = quadrants.iter().fold(1, |a, b| a * b);
    println!("{}", sum);
}

fn is_ok(x: &Result<i32, std::num::ParseIntError>) -> bool {
    match x {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn part2(lines: Vec<String>) {
    // let max_x: i32 = 11;
    // let max_y: i32 = 7;
    // let tree = init_tree();
    let mut nums: Vec<i32>;
    let mut map: Vec<Vec<Vec<(i32, i32)>>> = Vec::new();

    for x in 0..MAX_X as usize {
        map.push(Vec::new());
        for _ in 0..MAX_Y {
            map[x].push(Vec::new());
        }
    }
    for line in lines {
        nums = line
            .split('=')
            .flat_map(|x| x.split(' '))
            .flat_map(|x| x.split(','))
            .map(|x| x.parse::<i32>())
            .filter(|x: &Result<i32, std::num::ParseIntError>| is_ok(x))
            .map(|x| x.unwrap())
            .collect();
        map[nums[0] as usize][nums[1] as usize].push((nums[2], nums[3]));
    }
    let mut i = 0;
    while i < 2000000 {
        if i % 100 == 0 {
            println!("{}", i);
        }
        if is_tree(&map) {
            let mut input = String::new();
            println!("{}", i);

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
        }

        map = step(&map);

        i += 1
    }
    println!("{}", i);
}

fn is_tree(map: &Vec<Vec<Vec<(i32, i32)>>>) -> bool {
    let mut simple: Vec<Vec<char>> = map
        .iter()
        .map(|col| {
            col.iter()
                .map(|elems| if elems.len() != 0 { '#' } else { ' ' })
                .collect::<Vec<char>>()
        })
        .collect();
    simple = transpose(simple);

    for row in &simple {
        //found out from reddit what the tree looks like. so a little help on this one.
        if is_subsequence(&vec!['#'; 30], &row) {
            for col in &simple {
                println!("{:?}", col);
            }
            return true;
        }
    }
    false
}

fn step(map: &Vec<Vec<Vec<(i32, i32)>>>) -> Vec<Vec<Vec<(i32, i32)>>> {
    let mut new_map: Vec<Vec<Vec<(i32, i32)>>> = Vec::new();

    for x in 0..MAX_X as usize {
        new_map.push(Vec::new());
        for _ in 0..MAX_Y {
            new_map[x].push(Vec::new());
        }
    }
    for (x, col) in map.iter().enumerate() {
        for (y, elems) in col.iter().enumerate() {
            if elems.is_empty() {
                continue;
            } else {
                for elem in elems {
                    new_map[(x as i32 + elem.0).rem_euclid(MAX_X) as usize]
                        [(y as i32 + elem.1).rem_euclid(MAX_Y) as usize]
                        .push(*elem);
                }
            }
        }
    }
    new_map
}

fn transpose(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut transposed: Vec<Vec<char>> = vec![vec![' '; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            transposed[j][i] = matrix[i][j];
        }
    }

    transposed
}

fn is_subsequence(sub: &[char], main: &[char]) -> bool {
    let sub_len = sub.len();
    let main_len = main.len();

    // Slide a window over the main vector to check for the subsequence
    for i in 0..=main_len - sub_len {
        if main[i..i + sub_len] == *sub {
            return true;
        }
    }

    false
}
