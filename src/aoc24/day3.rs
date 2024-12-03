use regex::Regex;

pub fn part1(lines: Vec<String>) {
    let mut sum = 0;
    let re = Regex::new(r"^mul\([0-9][0-9]?[0-9]?,[0-9][0-9]?[0-9]?\)").unwrap(); //[0-9][0-9]?[0-9]?,[0-9][0-9]?[0-9]?)
    let str = lines.join("");
    for (i, c) in str.chars().enumerate() {
        let temp = &str[i..];
        if re.is_match(temp) {
            let num1: i32;
            let num2: i32;
            let commaidx: usize;
            match temp.find(",") {
                Some(idx) => {
                    commaidx = idx;
                    num1 = temp[4..idx].parse().unwrap();
                }
                none => {
                    continue;
                }
            }
            match temp.find(")") {
                Some(idx) => {
                    num2 = temp[(commaidx + 1)..idx].parse().unwrap();
                    sum = sum + num1 * num2;
                }
                none => {
                    continue;
                }
            }
        }
    }
    println!("{}", sum);
}

pub fn part2(lines: Vec<String>) {
    let mut go = true;
    let mut sum = 0;
    let re_mul = Regex::new(r"^mul\([0-9][0-9]?[0-9]?,[0-9][0-9]?[0-9]?\)").unwrap();
    let str = lines.join("");
    for (i, c) in str.chars().enumerate() {
        if go {
            let temp = &str[i..];
            if temp.starts_with("don't()") {
                go = false;
                continue;
            }
            if re_mul.is_match(temp) {
                let num1: i32;
                let num2: i32;
                let commaidx: usize;
                match temp.find(",") {
                    Some(idx) => {
                        commaidx = idx;
                        num1 = temp[4..idx].parse().unwrap();
                    }
                    none => {
                        continue;
                    }
                }
                match temp.find(")") {
                    Some(idx) => {
                        num2 = temp[(commaidx + 1)..idx].parse().unwrap();
                        sum = sum + num1 * num2;
                    }
                    none => {
                        continue;
                    }
                }
            }
        }else{
            let temp = &str[i..];
            if temp.starts_with("do()") {
                go = true;
                continue;
            }

        }
    }
    println!("{}", sum);
}
