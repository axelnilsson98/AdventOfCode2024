use std::collections::{HashSet, VecDeque};

pub fn part1(lines: Vec<String>) {
    let map_size = 71;
    let read_until = 1024;
    let mut map = vec![vec![true; map_size]; map_size];
    for i in 0..read_until{
        let cords: Vec<usize> = lines[i].split(",").map(|x| x.parse().unwrap()).collect();
        map[cords[1]][cords[0]] = false;
    };
    
    let mut pos: (usize, usize) = (0,0);
    let mut steps: i32 = 0;
    let mut to_handle: VecDeque<(usize, usize, i32)> = vec![(0,0,0)].into_iter().collect();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    print_map(&map, &visited);
    
    while !to_handle.is_empty(){
        (pos.0, pos.1, steps) = to_handle.pop_front().unwrap();
        if pos == (map_size-1, map_size-1){
            break;
        }
        for new_pos in valid_steps(&map, pos.0, pos.1){
            if visited.contains(&new_pos){continue;}
            visited.insert(new_pos);
            to_handle.push_back((new_pos.0, new_pos.1, steps +1));
        }
    }
    print_map(&map, &visited);
    println!("{}", steps);
}

fn valid_steps(map: &Vec<Vec<bool>>, i: usize, j: usize) -> Vec<(usize, usize)>{
    let mut  res: Vec<(usize, usize)> = Vec::new();
    for (dir_i, dir_j) in vec![(0,1), (1,0), (0,-1), (-1,0)]{
        let (ni, nj) = (i as i32 +dir_i, j as i32 + dir_j);
        if ni >= 0 && nj >= 0 && ni < map.len() as i32 && nj < map.len() as i32 && map[ni as usize][nj as usize]{
            res.push((ni as usize,nj as usize));
        }
    }

    res
}

fn print_map(map: &Vec<Vec<bool>>, visited: &HashSet<(usize, usize)>) {
    println!();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if visited.contains(&(i,j)) {
                print!("0");
            }else if map[i][j] {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
    println!();
}


pub fn part2(lines: Vec<String>) {
    let map_size = 71;
    let mut lower_bound = 1024;
    let mut upper_bound = lines.len();
    let mut current: usize = 0;
    let mut found_solution = true;
    'outer: while upper_bound - lower_bound > 1{
        let mut map = vec![vec![true; map_size]; map_size];
        println!("diff: {}", ((upper_bound-lower_bound) as f32 /2.0).ceil());
        current  = lower_bound + ((upper_bound-lower_bound) as f32 /2.0).ceil() as usize;
        println!("{}", current);
        for i in 0..current{
            let cords: Vec<usize> = lines[i].split(",").map(|x| x.parse().unwrap()).collect();
            map[cords[1]][cords[0]] = false;
        };

        let mut pos: (usize, usize) = (0,0);
        let mut steps: i32;
        let mut to_handle: VecDeque<(usize, usize, i32)> = vec![(0,0,0)].into_iter().collect();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        
        while !to_handle.is_empty(){
            (pos.0, pos.1, steps) = to_handle.pop_front().unwrap();
            if pos == (map_size-1, map_size-1){
                lower_bound = current;
                println!("found solution");
                found_solution = true;
                continue 'outer;
            }
            for new_pos in valid_steps(&map, pos.0, pos.1){
                if visited.contains(&new_pos){continue;}
                visited.insert(new_pos);
                to_handle.push_back((new_pos.0, new_pos.1, steps +1));
            }
        }
        println!("didnt find solution. lowering bound");
        upper_bound = current;
        found_solution = false;
    }
    if !found_solution{
        current-=1;
    }
    println!("{}", current);
    println!("{}", lines[current]);
}