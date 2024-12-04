use std::usize;

type Dimensions = (usize, usize);

pub fn part1(lines: Vec<String>) {
    let dim: Dimensions = (lines.len(), lines[0].len());
    let matrix: Vec<Vec<char>> = lines.iter().map(|str| str.chars().collect()).collect();
    let mut num = 0;
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
fn num_xmas(l: usize, c: usize, m: &Vec<Vec<char>>, d: Dimensions) -> i32 {
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

    let num = dirs
        .iter()
        .filter(|&&(ll, cc)| {
            let l1 = get_cord(l, ll, 1);
            let c1 = get_cord(c, cc, 1);
            let l2 = get_cord(l, ll, 2);
            let c2 = get_cord(c, cc, 2);
            let l3 = get_cord(l, ll, 3);
            let c3 = get_cord(c, cc, 3);

            is_within_bounds(l3, c3, d)
                && m[l1 as usize][c1 as usize] == 'M'
                && m[l2 as usize][c2 as usize] == 'A'
                && m[l3 as usize][c3 as usize] == 'S'
        })
        .count() as i32;
    num
}

fn is_within_bounds(x: i32, y: i32, d: Dimensions) -> bool {
    x >= 0 && x < d.0 as i32 && y >= 0 && y < d.1 as i32
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
    let dirs: [((i32, i32), (i32, i32)); 4] = [
        ((1, 1), (-1, -1)),
        ((-1, 1), (1, -1)),
        ((-1, -1), (1, 1)),
        ((1, -1), (-1, 1)),
    ];
    let num: i32 = dirs
        .iter()
        .filter(|((ml, mc), (sl, sc))| {
            is_inside(l, c, d)
                && m[get_cord(l, *ml, 1) as usize][get_cord(c, *mc, 1) as usize] == 'M'
                && m[get_cord(l, *sl, 1) as usize][get_cord(c, *sc, 1) as usize] == 'S'
        })
        .count() as i32;
    num == 2
}

fn get_cord(x: usize, dir: i32, steps: i32) -> i32 {
    x as i32 + steps * dir
}

fn is_inside(l: usize, c: usize, d: (usize, usize)) -> bool {
    get_cord(l, -1, 1) >= 0
        && get_cord(l, 1, 1) < d.0 as i32
        && get_cord(c, -1, 1) >= 0
        && get_cord(c, 1, 1) < d.1 as i32
}
