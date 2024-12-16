pub fn part1(lines: Vec<String>) {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut robo_pos: (usize, usize) = (0, 0);

    let mut i = 0;

    while !lines[i].is_empty() {
        if lines[i].contains("@") {
            robo_pos = (i, lines[i].find("@").unwrap());
        }
        map.push(lines[i].chars().collect());
        i += 1;
    }
    // print_map(&map);
    i += 1;
    while i < lines.len() {
        lines[i].chars().for_each(|c| {
            let dir: (i32, i32) = match c {
                'v' => (1, 0),
                '<' => (0, -1),
                '>' => (0, 1),
                '^' => (-1, 0),
                a => {
                    println!("illegal char found: {}", a);
                    (0, 0)
                }
            };
            let ret_val = step1(map.clone(), dir, robo_pos);
            map = ret_val.1;
            if ret_val.0 {
                robo_pos = (
                    (robo_pos.0 as i32 + dir.0) as usize,
                    (robo_pos.1 as i32 + dir.1) as usize,
                );
            }

            println!("return val: {}", ret_val.0);
            // print_map(&map);
        });
        i += 1;
    }
    let mut sum = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if *char == 'O' {
                sum += 100 * i + j;
            }
        }
    }
    println!("{}", sum);
}

fn step1(mut map: Vec<Vec<char>>, dir: (i32, i32), pos: (usize, usize)) -> (bool, Vec<Vec<char>>) {
    // println!("pos: {:?}", pos);
    // println!("dir: {:?}", dir);
    let next_pos = (
        (pos.0 as i32 + dir.0) as usize,
        (pos.1 as i32 + dir.1) as usize,
    );
    let next_char = map[next_pos.0][next_pos.1];
    let ret_val = match next_char {
        '.' => {
            println!(
                "next: {}, this: {}",
                map[next_pos.0][next_pos.1], map[pos.0][pos.1]
            );
            map[next_pos.0][next_pos.1] = map[pos.0][pos.1];
            map[pos.0][pos.1] = '.';
            return (true, map);
        }
        '#' => return (false, map),
        'O' => step1(map, dir, next_pos),
        a => {
            println!("illegal char found: {}", a);
            (false, map)
        }
    };
    let mov = ret_val.0;
    map = ret_val.1;
    if mov {
        println!(
            "this: {}, next: {}",
            map[pos.0][pos.1], map[next_pos.0][next_pos.1]
        );
        map[next_pos.0][next_pos.1] = map[pos.0][pos.1];
        map[pos.0][pos.1] = '.';
    }
    (mov, map)
}

fn print_map(map: &Vec<Vec<char>>) {
    println!();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            print!(" {} ", map[i][j]);
        }
        println!();
    }
    println!();
}

pub fn part2(lines: Vec<String>) {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut robo_pos: (usize, usize) = (0, 0);

    let mut i = 0;

    while !lines[i].is_empty() {
        if lines[i].contains("@") {
            robo_pos = (i, 2 * lines[i].find("@").unwrap());
        }
        map.push(
            lines[i]
                .chars()
                .flat_map(|c| match c {
                    '#' => vec!['#', '#'],
                    '.' => vec!['.', '.'],
                    '@' => vec!['@', '.'],
                    'O' => vec!['[', ']'],
                    a => {
                        println!("illegal character {}", a);
                        vec![]
                    }
                })
                .collect(),
        );
        i += 1;
    }
    i += 1;
    while i < lines.len() {
        lines[i].chars().for_each(|c| {
            let mut dir: (i32, i32) = (0, 0);
            let mut vertical = false;
            let ret_val = match c {
                'v' => {
                    dir = (1, 0);
                    vertical = true;
                    check_if_step_vert(&map, dir, robo_pos, true)
                }
                '<' => {
                    dir = (0, -1);
                    check_if_step_hor(&map, dir, robo_pos)
                }
                '>' => {
                    dir = (0, 1);
                    check_if_step_hor(&map, dir, robo_pos)
                }
                '^' => {
                    dir = (-1, 0);
                    vertical = true;
                    check_if_step_vert(&map, dir, robo_pos, true)
                }
                a => {
                    println!("illegal char found: {}", a);
                    false
                }
            };
            if ret_val {
                let next_pos = (
                    (robo_pos.0 as i32 + dir.0) as usize,
                    (robo_pos.1 as i32 + dir.1) as usize,
                );
                let next_char = map[next_pos.0][next_pos.1];
                if next_char == '[' && vertical {
                    map = step2(
                        map.clone(),
                        dir,
                        (next_pos.0, next_pos.1 + 1),
                        ']',
                        vertical,
                    );
                    map[next_pos.0][next_pos.1 + 1] = '.';
                } else if next_char == ']' && vertical {
                    map = step2(
                        map.clone(),
                        dir,
                        (next_pos.0, next_pos.1 - 1),
                        '[',
                        vertical,
                    );
                    map[next_pos.0][next_pos.1 - 1] = '.';
                }
                map = step2(map.clone(), dir, robo_pos, '@', vertical);

                map[robo_pos.0][robo_pos.1] = '.';

                robo_pos = next_pos;
            } else {
            }

        });
        i += 1;
    }
    print_map(&map);

    let mut sum = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if *char == '[' {
                sum += 100 * i + j;
            }
        }
    }
    println!("{}", sum);
}

fn step2(
    mut map: Vec<Vec<char>>,
    dir: (i32, i32),
    pos: (usize, usize),
    c: char,
    vertical: bool,
) -> Vec<Vec<char>> {
    let next_pos = (
        (pos.0 as i32 + dir.0) as usize,
        (pos.1 as i32 + dir.1) as usize,
    );

    let tmp = map[next_pos.0][next_pos.1];

    map[next_pos.0][next_pos.1] = c;

    if tmp == '[' {
        if c == ']' && vertical {
            let neigboor = map[next_pos.0][next_pos.1 + 1];
            map = step2(map, dir, (next_pos.0, next_pos.1 + 1), neigboor, vertical);
            map[next_pos.0][next_pos.1 + 1] = '.';
        }
        map = step2(map, dir, next_pos, tmp, vertical);
    } else if tmp == ']' {
        if c == '[' && vertical {
            let neigboor = map[next_pos.0][next_pos.1 - 1];
            map = step2(map, dir, (next_pos.0, next_pos.1 - 1), neigboor, vertical);
            map[next_pos.0][next_pos.1 - 1] = '.';
        }
        map = step2(map, dir, next_pos, tmp, vertical);
    } else if tmp == '.' {
    } else {
        panic!("seen a weird c");
    }
    map
}

fn check_if_step_vert(
    map: &Vec<Vec<char>>,
    dir: (i32, i32),
    pos: (usize, usize),
    first: bool,
) -> bool {
    let next_pos = (
        (pos.0 as i32 + dir.0) as usize,
        (pos.1 as i32 + dir.1) as usize,
    );
    let next_char = map[next_pos.0][next_pos.1];
    if first {
        let ret_val = match next_char {
            '.' => return true,
            '#' => return false,
            '[' => (
                check_if_step_vert(map, dir, next_pos, false),
                check_if_step_vert(map, dir, (next_pos.0, next_pos.1 + 1), false),
            ),
            ']' => (
                check_if_step_vert(map, dir, (next_pos.0, next_pos.1 - 1), false),
                check_if_step_vert(map, dir, next_pos, false),
            ),
            a => {
                println!("illegal char found: {}", a);
                (false, false)
            }
        };
        ret_val.0 && ret_val.1
    } else {
        let last_char = map[pos.0][pos.1];
        let ret_val = match next_char {
            '.' => return true,
            '#' =>  return false,
            '[' => {
                if last_char == ']' {
                    if !check_if_step_vert(map, dir, (next_pos.0, next_pos.1 + 1), false) {
                        return false;
                    }
                }
                check_if_step_vert(map, dir, next_pos, false)
            }
            ']' => {
                if last_char == '[' {

                    if !check_if_step_vert(map, dir, (next_pos.0, next_pos.1 - 1), false) {

                        return false;
                    }
                }
                check_if_step_vert(map, dir, next_pos, false)
            }
            a => {
                println!("illegal char found: {}", a);
                false
            }
        };
        ret_val
    }
}

fn check_if_step_hor(map: &Vec<Vec<char>>, dir: (i32, i32), pos: (usize, usize)) -> bool {
    let next_pos = (
        (pos.0 as i32 + dir.0) as usize,
        (pos.1 as i32 + dir.1) as usize,
    );
    let next_char = map[next_pos.0][next_pos.1];
    let ret_val = match next_char {
        '.' => return true,
        '#' => return false,
        '[' => check_if_step_hor(map, dir, next_pos),
        ']' => check_if_step_hor(map, dir, next_pos),
        a => {
            println!("illegal char found: {}", a);
            false
        }
    };
    ret_val
}
