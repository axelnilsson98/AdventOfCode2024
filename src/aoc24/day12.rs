use std::collections::{HashSet, VecDeque};

pub fn part1(lines: Vec<String>) {
    let map: Vec<Vec<char>> = lines
        .iter()
        .map(|xs| xs.chars().map(|x| x).collect())
        .collect();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut local_visited: HashSet<(usize, usize)> = HashSet::new();
    let mut current_plot: HashSet<(usize, usize)> = HashSet::new();
    let mut current_char: char = '0';
    let mut to_visit: Vec<(usize, usize)> = Vec::new();
    let mut sum = 0;

    to_visit.push((0, 0));

    'outer: while visited.len() < map.len() * map[0].len() {
        if to_visit.is_empty() {
            let fences = calc_fences(&current_plot, &map);
            sum += fences * current_plot.len() as i32;

            current_plot.clear();
            current_char = '0';
            local_visited.clear();

            for ni in 0..map.len() {
                'inner: for nj in 0..map[ni].len() {
                    if visited.contains(&(ni, nj)) {
                        continue 'inner;
                    } else {
                        to_visit.push((ni, nj));
                        continue 'outer;
                    }
                }
            }
        }
        let (i, j) = to_visit.pop().unwrap();
        if local_visited.contains(&(i, j)) {
            continue 'outer;
        }
        local_visited.insert((i, j));
        if visited.contains(&(i, j)) {
            continue 'outer;
        }
        if current_char == '0' {
            current_char = map[i][j];
            // println!("starting {}", current_char);
        }
        if map[i][j] == current_char {
            visited.insert((i, j));
            current_plot.insert((i, j));
        } else {
            continue 'outer;
        }
        for (ni, nj) in valid_indexes(i, j, &map) {
            to_visit.push((ni, nj));
        }
    }
    let fences = calc_fences(&current_plot, &map);
    sum += fences * current_plot.len() as i32;
    println!(
        "fences: {}, sum: {}",
        fences,
        fences * current_plot.len() as i32
    );

    println!("{}", sum);
}

fn valid_indexes(i: usize, j: usize, map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    if i != 0 {
        res.push((i - 1, j));
    }
    if j != 0 {
        res.push((i, j - 1));
    }
    if i < (map.len() - 1) {
        res.push((i + 1, j));
    }
    if j < (map[0].len() - 1) {
        res.push((i, j + 1));
    }
    res
}

fn calc_fences(plots: &HashSet<(usize, usize)>, map: &Vec<Vec<char>>) -> i32 {
    let mut res: i32 = 0;

    plots.iter().for_each(|(i, j)| {
        let neigboors: HashSet<(usize, usize)> =
            valid_indexes(*i, *j, map).iter().cloned().collect();
        let intersection = plots
            .intersection(&neigboors)
            .collect::<HashSet<&(usize, usize)>>()
            .len() as i32;
        println!("{}", intersection);
        res += 4 - intersection;
    });
    res
}

pub fn part2(lines: Vec<String>) {
    let map: Vec<Vec<char>> = lines
        .iter()
        .map(|xs| xs.chars().map(|x| x).collect())
        .collect();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut local_visited: HashSet<(usize, usize)> = HashSet::new();
    let mut current_plot: HashSet<(usize, usize)> = HashSet::new();

    let mut current_char: char = '0';
    let mut to_visit: Vec<(usize, usize)> = Vec::new();
    let mut sum = 0;

    to_visit.push((0, 0));

    'outer: while visited.len() < map.len() * map[0].len() {
        if to_visit.is_empty() {
            let corners = calc_corners(&current_plot, &map);
            sum += corners * current_plot.len() as i32;

            current_plot.clear();
            current_char = '0';
            local_visited.clear();

            for ni in 0..map.len() {
                'inner: for nj in 0..map[ni].len() {
                    if visited.contains(&(ni, nj)) {
                        continue 'inner;
                    } else {
                        to_visit.push((ni, nj));
                        continue 'outer;
                    }
                }
            }
        }
        let (i, j) = to_visit.pop().unwrap();
        if local_visited.contains(&(i, j)) {
            continue 'outer;
        }
        local_visited.insert((i, j));
        if visited.contains(&(i, j)) {
            continue 'outer;
        }
        if current_char == '0' {
            current_char = map[i][j];
        }
        if map[i][j] == current_char {
            visited.insert((i, j));
            current_plot.insert((i, j));
        } else {
            continue 'outer;
        }
        for (ni, nj) in valid_indexes(i, j, &map) {
            to_visit.push((ni, nj));
        }
    }
    let corners = calc_corners(&current_plot, &map);
    sum += corners * current_plot.len() as i32;


    println!("{}", sum);
}

fn calc_corners(plot: &HashSet<(usize, usize)>, map: &Vec<Vec<char>>) -> i32 {
    let mut corners = 0;
    plot.iter().for_each(|(i, j)|{
        let tmp: Vec<(i32, i32)> = vec![(*i as i32 +1 ,*j as i32), (*i as i32,*j as i32+1), (*i as i32-1, *j as i32), (*i as i32, *j as i32 -1)];
        let mut dirs: VecDeque<&(i32, i32)> = tmp.iter().collect();
        let c = get(map, *i as i32, *j as i32);
        for _ in 0..4{
            let dir1 = dirs.pop_front().unwrap();
            let dir2 = dirs.pop_front().unwrap();
            let c1 = get(map,dir1.0, dir1.1);
            let c2 = get(map,dir2.0, dir2.1);
            let c3 = get(map,dir2.0 + dir1.0 - *i as i32, dir2.1 + dir1.1 - *j as i32);


            if c1 != c && c2 != c{
                corners += 1;
            }else if c1 == c2 && c1 == c && c3 != c{
                corners += 1;
            }
            dirs.push_back(dir1);
            dirs.push_front(dir2);
        }
    });


    corners
}

fn get(map: &Vec<Vec<char>>, i: i32, j: i32) -> char {
    if i < 0 || i >= map.len() as i32 || j < 0 || j >= map[0].len() as i32{
        '0'
    }else{
        map[i as usize][j as usize]
    }
}

