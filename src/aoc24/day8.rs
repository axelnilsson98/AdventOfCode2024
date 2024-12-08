use std::{collections::HashSet, usize};

pub fn part1(lines: Vec<String>) {
    let mut map: Vec<(char, usize, usize)> = Vec::new();
    let mut freq_set: HashSet<char> = HashSet::new();
    let i_max = lines.len() as i32;
    let mut j_max = 0;
    for (i, line) in lines.iter().enumerate() {
        j_max = line.len() as i32;
        for (j, char) in line.chars().enumerate() {
            if char != '.' {
                map.push((char, i, j));
                freq_set.insert(char);
            }
        }
    }
    println!("{}, {}", i_max, j_max);
    let mut nbr = 0;
    let mut found: HashSet<(i32, i32)> = HashSet::new();
    for freq in freq_set {
        let temp_map: Vec<&(char, usize, usize)> =
            map.iter().filter(|(char, i, j)| *char == freq).collect();
        for (k, (c1, i1, j1)) in temp_map.iter().enumerate() {
            for l in (k + 1)..temp_map.len() {
                let (c2, i2, j2) = temp_map[l];
                let idiff: i32 = *i2 as i32 - *i1 as i32;
                let jdiff: i32 = *j2 as i32 - *j1 as i32;
                let i3: i32 = *i1 as i32 - idiff;
                let i4: i32 = *i2 as i32 + idiff;
                let j3: i32 = *j1 as i32 - jdiff;
                let j4: i32 = *j2 as i32 + jdiff;

                if !is_not_valid(i3, j3, i_max, j_max) && !found.contains(&(i3, j3)){
                    // println!("{}, {}", i3, j3);
                    found.insert((i3, j3));
                    nbr += 1;
                }
                if !is_not_valid(i4, j4, i_max, j_max) && !found.contains(&(i4, j4)){
                    // println!("{}, {}", i4, j4);
                    found.insert((i4,j4));
                    nbr += 1;
                }
            }
        }
    }

    println!("nbr: {}", nbr);
}

fn is_not_valid(i: i32, j: i32, i_max: i32, j_max: i32) -> bool {
    // println!("{}, {}", i, j);
    i < 0 || i >= i_max || j < 0 || j >= j_max
}
pub fn part2(lines: Vec<String>) {}
