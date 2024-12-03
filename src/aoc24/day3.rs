use regex::Regex;

pub fn part1(lines: Vec<String>) {
    let mut sum = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let str = lines.join("");
    for caps in re.captures_iter(&str) {
        if let (Some(n1), Some(n2)) = (caps.get(1), caps.get(2)) {
            sum += n1.as_str().parse::<i32>().unwrap() * n2.as_str().parse::<i32>().unwrap();
        }
    }
    println!("{}", sum);
}

pub fn part2(lines: Vec<String>) {
    let mut go = true;
    let mut sum: i64 = 0;
    let re_mul = Regex::new(r"^mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let re_command = Regex::new(r"^(don't\(\)|do\(\))").unwrap();
    let str = lines.join("");
    let mut i = 0;
    while i < str.len() {
        let temp = &str[i..];
        if let Some(command_match) = re_command.find(temp) {
            let command = &temp[command_match.start()..command_match.end()];
            if command == "don't()" {
                go = false;
            } else if command == "do()" {
                go = true;
            }
            i += command_match.end();
            continue;
        }
        if go {
            if let Some(mul_match) = re_mul.captures(temp) {
                if let (Some(n1), Some(n2)) = (mul_match.get(1), mul_match.get(2)) {
                    sum +=
                        n1.as_str().parse::<i64>().unwrap() * n2.as_str().parse::<i64>().unwrap();
                }
                i += mul_match.get(1).unwrap().end();
                continue;

            }
        }
        i += 1;
    }
    println!("{}", sum);
}
