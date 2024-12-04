use std::usize;

pub fn part1(lines: Vec<String>) {
    let mut num = 0;
    let dim: (usize, usize) = (lines.len(), lines[0].len());
    let matrix: Vec<Vec<char>> = lines.into_iter().map(|str| str.chars().collect()).collect();
    for l in 0..matrix.len() {
        for c in 0..matrix[l].len() {
            if matrix[l][c] == 'X' {
                let lnum = num_xmas(l, c, &matrix, dim);
                println!("found {} xmas at {},{}", lnum, l, c);
                num += lnum;
            }
        }
    }
    println!("{}", num);
}
fn num_xmas(l: usize, c: usize, m: &Vec<Vec<char>>, d: (usize, usize)) -> i32 {
    let mut num = 0;
    let dirs: [(i32, i32); 8] = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    // let mut dir: (i32, i32);
    for (ll, cc) in dirs {
        if get_cord(l, ll, 3) >= 0
            && get_cord(l, ll, 3) < d.0 as i32
            && get_cord(c, cc, 3) >= 0
            && get_cord(c, cc, 3) < d.1 as i32
        {
            if m[get_cord(l, ll, 1) as usize][get_cord(c, cc, 1) as usize] == 'M'
                && m[get_cord(l, ll, 2) as usize][get_cord(c, cc, 2) as usize] == 'A'
                && m[get_cord(l, ll, 3) as usize][get_cord(c, cc, 3) as usize] == 'S'
            {
                num += 1;
            }
        }
    }
    num
}

pub fn part2(lines: Vec<String>) {
    let mut num = 0;
    let dim: (usize, usize) = (lines.len(), lines[0].len());
    let matrix: Vec<Vec<char>> = lines.into_iter().map(|str| str.chars().collect()).collect();

    for l in 0..matrix.len() {
        for c in 0..matrix[l].len() {
            if matrix[l][c] == 'A' && is_x_mas(l, c, &matrix, dim) {
                println!("found xmas at {},{}", l, c);
                num += 1;
            }
        }
    }
    println!("{}", num);
}

fn is_x_mas(l: usize, c: usize, m: &Vec<Vec<char>>, d: (usize, usize)) -> bool {
    let dirs: [((i32, i32), (i32, i32)); 4] = [((1, 1), (-1, -1)), ((-1, 1), (1, -1)), ((-1, -1), (1, 1)), ((1, -1), (-1, 1))];
    let mut num = 0;// let mut dir: (i32, i32);
    for ((ml, mc), (sl, sc)) in dirs {
        if is_inside(l, c, d, (ml, mc) )&& is_inside(l, c, d, (sl, sc))
        {
            if m[get_cord(l, ml, 1) as usize][get_cord(c, mc, 1) as usize] == 'M'
                && m[get_cord(l, sl, 1) as usize][get_cord(c, sc, 1) as usize] == 'S'
            {
                // println!("xmas found dirs: ml: {}, mc: {}, sl: {}, sc: {}", ml, mc, sl, sc);
                num +=1;
            }
        }
    }
    num ==2
}

fn get_cord(x: usize, dir: i32, steps: i32) -> i32 {
    // println!("x: {}, dir: {}, steps: {}", x, dir, steps);
    x as i32 + steps * dir
}

fn is_inside(l: usize, c: usize, d: (usize, usize), dir:(i32, i32))-> bool{
    get_cord(l, dir.0, 1) >= 0
    && get_cord(l, dir.0, 1) < d.0 as i32
    && get_cord(c, dir.1, 1) >= 0
    && get_cord(c, dir.1, 1) < d.1 as i32
}
