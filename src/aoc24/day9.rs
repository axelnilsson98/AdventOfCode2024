use std::collections::HashSet;

pub fn part1(lines: Vec<String>) {
    let nums: Vec<i64> = lines[0]
        .chars()
        .map(|x| x.to_string().parse().unwrap())
        .collect();
    let mut disk: Vec<i64> = Vec::new();

    let mut file = true;
    let mut id = 0;

    nums.iter().for_each(|num|{
        let to_push:Vec<i64> = if file {id += 1; vec![id-1; *num as usize]} else {vec![-1; *num as usize]};
        disk.extend(to_push);
        file = !file;
    });



    let mut new_disk = Vec::new();
    let mut j = disk.len();
    for (i, num) in disk.iter().enumerate() {
        if i >= (j) {
            break;
        } else if *num == -1 {
            for k in (0..j).rev() {
                if disk[k] != -1 {
                    new_disk.push(disk[k]);
                    j = k;
                    break;
                }
            }
        } else {
            new_disk.push(*num);
        }
    }
    let sum: i64 = new_disk
        .iter()
        .enumerate()
        .map(|(idx, num)| idx as i64 * *num as i64)
        .sum::<i64>();
    println!("{}", sum);
}

pub fn part2(lines: Vec<String>) {
    let nums: Vec<i64> = lines[0]
        .chars()
        .map(|x| x.to_string().parse().unwrap())
        .collect();
    let mut disk: Vec<i64> = Vec::new();

    let mut file = true;
    let mut id = 0;
    nums.iter().for_each(|num|{
        let to_push:Vec<i64> = if file {id += 1; vec![id-1; *num as usize]} else {vec![-1; *num as usize]};
        disk.extend(to_push);
        file = !file;
    });


    let mut new_disk: Vec<i64> = disk.clone();
    let mut moved_num: HashSet<i64> = HashSet::new();
    for (i, num) in disk.iter().enumerate().rev() {
        if moved_num.contains(num) {
            continue;
        }
        if *num != -1 {
            let num_to_move = disk.iter().filter(|x| *x == num).count();
            moved_num.insert(*num);
            let mut j = 0;
            'inner: while j < i {
                if new_disk[j] != -1 {
                    j += 1;
                    continue;
                }
                let num_free = get_num(&new_disk[j..], -1);
                if num_free >= num_to_move as i64 {
                    for k in j..(j + (num_to_move)) {
                        new_disk[k] = *num;
                        new_disk[i] = -1;
                    }
                    for k in (i - num_to_move + 1)..i + 1 {
                        new_disk[k] = -1
                    }
                    break 'inner;
                }
                j += 1;
            }
        }

    }

    let sum: i64 = new_disk
        .iter()
        .enumerate()
        .map(|(idx, num)| {
            if *num != -1 {
                idx as i64 * *num as i64
            } else {
                0
            }
        })
        .sum::<i64>();
    println!("{}", sum);
}

fn get_num(slice: &[i64], token: i64) -> i64 {
    let mut num = 0;
    while slice[num] == token {
        num += 1;
    }
    num as i64
}
