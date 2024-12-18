use rand::Rng;


pub fn part1(lines: Vec<String>) {
    let mut registers = vec![0, 0, 0];
    for i in 0..2 {
        registers[i] = lines[i].split(" ").collect::<Vec<&str>>()[2]
            .parse()
            .unwrap();
    }
    let instructions: Vec<i64> = lines[4].split(" ").collect::<Vec<&str>>()[1]
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{:?}, {:?}", registers, instructions);

    let mut ip = 0;
    let mut out: Vec<i64> = Vec::new();
    while ip < instructions.len() {
        let opcode = instructions[ip];
        let literal = instructions[ip + 1];
        let combo = combo(&registers, literal);


        match opcode {
            0 => {
                registers[0] = registers[0] >> combo; // tmp;
            }
            1 => {
                registers[1] = registers[1] ^ literal;
            }
            2 => {
                registers[1] = combo & 7;
            }
            3 => {
                if registers[0] != 0 {
                    ip = literal as usize;
                    continue;
                }
            }
            4 => {
                registers[1] = registers[1] ^ registers[2];
            }
            5 => {
                out.push(combo & 7);
            }
            6 => {
                registers[1] = registers[0] >> combo; // tmp;
            }
            7 => {
                registers[2] = registers[0] >> combo; // tmp;
            }
            _ => panic!("illegal opcode {}", opcode),
        }
        ip += 2;
    }
    let out_string: String = out
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("{}", out_string);
}

fn combo(registers: &Vec<i64>, literal: i64) -> i64 {
    if literal >= 0 && literal <= 3 {
        return literal;
    } else {
        match literal {
            4 => registers[0],
            5 => registers[1],
            6 => registers[2],
            _ => panic!("illegal combo literal: {}", literal),
        }
    }
}

pub fn part2(lines: Vec<String>) {
    let mut registers = vec![0, 0, 0];
    for i in 0..2 {
        registers[i] = lines[i].split(" ").collect::<Vec<&str>>()[2]
            .parse()
            .unwrap();
    }
    let init_registers = registers.clone();
    let instructions: Vec<i64> = lines[4].split(" ").collect::<Vec<&str>>()[1]
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{:?}, {:?}", registers, instructions);

    let mut ip;
    let mut out: Vec<i64> = Vec::new();
    // the answer needs to be in the order of this number to match the correct amount of output digits.
    //35184372088960
    // gives correct but is to big
    //136904921131418 gives correct but is to big
    //136904920885658
    //136904920099226 // this is correct
    let pattern = "????????0???????0101001001100???010100110011010";
    let mut try_these = generate_matching_numbers(pattern, 10000000);
    println!("done generating");
    try_these.sort();
    println!("running simulation");
    let mut a = 136904920099226;
    //               1369049036000000
    //               136904921131418
    while out != instructions {
        if a % 1000000 == 0 {
            println!("{}", a)
        }
        a -= 1;

        out.clear();
        ip = 0;
        registers = init_registers.clone();
        registers[0] = a;//try_these[a];

        'program: while ip < instructions.len() {
            for (i, x) in out.iter().enumerate() {
                if i == 15 {
                }
                if *x != instructions[i] {
                    break 'program;
                }
            }
            let opcode: i64 = instructions[ip];
            let literal = instructions[ip + 1];
            let combo = combo(&registers, literal);

            match opcode {
                0 => {
                    registers[0] = registers[0] >> combo; 
                }
                1 => {
                    registers[1] = registers[1] ^ literal;
                }
                2 => {
                    registers[1] = combo & 7;
                }
                3 => {
                    if registers[0] != 0 {
                        ip = literal as usize;
                        continue;
                    }
                }
                4 => {
                    registers[1] = registers[1] ^ registers[2];
                }
                5 => {
                    out.push(combo & 7);
                }
                6 => {
                    registers[1] = registers[0] >> combo; 
                }
                7 => {
                    registers[2] = registers[0] >> combo; 
                }
                _ => panic!("illegal opcode {}", opcode),
            }
            ip += 2;
        }
    }
        println!("out: {:?}", out);
        println!("ins: {:?}", instructions);
        println!("a: {}", a);//try_these[a]);
    }



fn generate_matching_numbers(pattern: &str, count: usize) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    let mut matching_numbers = Vec::new();
    let mut i = 0;
    while i < count {
        let mut binary_str = String::new();

        for ch in pattern.chars() {
            match ch {
                '?' => binary_str.push(if rng.gen_bool(0.5) { '1' } else { '0' }),
                '0' | '1' => binary_str.push(ch),
                _ => panic!("Invalid character in pattern: {}", ch),
            }
        }

        // Convert the binary string to a number
        let number = u64::from_str_radix(&binary_str, 2).unwrap();
        if number >= 136904921131418{
            continue;
        }
        matching_numbers.push(number as i64);
        i+=1;
    }

    matching_numbers
}
