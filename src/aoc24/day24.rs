use itertools::Itertools;
use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};

// pub fn part1(lines: Vec<String>) {
//     let mut constants: HashMap<String, bool> = HashMap::new();

//     let mut i = 0;
//     let mut line = &lines[i];

//     // parse constants
//     while line != "" {
//         let signal: Vec<&str> = line.split(": ").collect();
//         if signal[1] == "1" {
//             constants.insert(signal[0].to_string(), true);
//         } else {
//             constants.insert(signal[0].to_string(), false);
//         }

//         i += 1;
//         line = &lines[i];
//     }

//     println!("{:?}", constants);

//     let mut output: Vec<&str> = Vec::new();
//     let mut gates: HashMap<&str, (&str, &str, usize)> = HashMap::new();
//     i += 1;
//     while i < lines.len() {
//         line = &lines[i];
//         let gate: Vec<&str> = line.split(" ").collect();
//         let t: usize = match gate[1] {
//             "XOR" => 0,
//             "OR" => 1,
//             "AND" => 2,
//             c => panic!("{}", c),
//         };
//         gates.insert(gate[4], (gate[0], gate[2], t));

//         if gate[4].starts_with("z") {
//             output.push(gate[4]);
//         }
//         i += 1;
//     }
//     output.sort();
//     println!("{:?}", output);
//     let mut num: Vec<bool> = Vec::new();

//     for o in output {
//         let res: bool;
//         (constants, res) = calc(constants, o.to_string(), &gates, 1);

//         num.push(res);
//     }

//     println!("{:?}", num);
//     let number = u64::from_str_radix(&num.iter().map(|x| *x as usize).rev().join(""), 2).unwrap();
//     println!("{}", number);
// }

fn calc(
    mut constants: HashMap<String, bool>,
    input: String,
    gates: &HashMap<&str, (&str, &str, usize)>,
    depth: usize,
) -> Result<(HashMap<String, bool>, bool), String> {
    if depth > 4 {
        Err("too deep".to_string())
    } else if constants.contains_key(&input) {
        let res = *constants.get(&input).unwrap();
        Ok((constants, res))
    } else {
        let gate = gates.get(&input[..]).unwrap();
        let op1: bool;
        let op2: bool;
        let t = calc(constants, gate.0.to_string(), gates, depth + 1);
        match t {
            Ok((c, o)) => {
                op1 = o;
                constants = c
            }
            Err(s) => return Err(s),
        }
        let t = calc(constants, gate.0.to_string(), gates, depth + 1);
        match t {
            Ok((c, o)) => {
                op2 = o;
                constants = c
            }
            Err(s) => return Err(s),
        }
        let res = match gate.2 {
            0 => op1 != op2,
            1 => op1 || op2,
            2 => op1 && op2,
            x => panic!("{}", x),
        };

        constants.insert(input, res);

        Ok((constants, res))
    }
}

pub fn part2(lines: Vec<String>) {
    let mut constants: HashMap<String, bool> = HashMap::new();

    let mut signals: Vec<String> = Vec::new();
    let mut i = 0;
    let mut line = &lines[i];
    let mut x: Vec<bool> = Vec::new();
    let mut y: Vec<bool> = Vec::new();

    // parse constants
    while line.starts_with("x") {
        let signal: Vec<&str> = line.split(": ").collect();
        if signal[1] == "1" {
            constants.insert(signal[0].to_string(), true);
            x.push(true);
        } else {
            constants.insert(signal[0].to_string(), false);
            x.push(false);
        }

        i += 1;
        line = &lines[i];
    }
    while line.starts_with("y") {
        let signal: Vec<&str> = line.split(": ").collect();
        if signal[1] == "1" {
            constants.insert(signal[0].to_string(), true);
            y.push(true);
        } else {
            constants.insert(signal[0].to_string(), false);
            y.push(false);
        }

        i += 1;
        line = &lines[i];
    }

    println!("{:?}", constants);

    let mut z: Vec<&str> = Vec::new();
    let mut gates: HashMap<&str, (&str, &str, usize)> = HashMap::new();
    i += 1;
    while i < lines.len() {
        line = &lines[i];
        let gate: Vec<&str> = line.split(" ").collect();
        let t: usize = match gate[1] {
            "XOR" => 0,
            "OR" => 1,
            "AND" => 2,
            c => panic!("{}", c),
        };
        gates.insert(gate[4], (gate[0], gate[2], t));
        signals.push(gate[4].to_string());

        if gate[4].starts_with("z") {
            z.push(gate[4]);
        }
        i += 1;
    }
    z.sort();

    let x = u64::from_str_radix(&x.iter().map(|x| *x as usize).rev().join(""), 2).unwrap();
    let y = u64::from_str_radix(&y.iter().map(|x| *x as usize).rev().join(""), 2).unwrap();
    let goal_z = format!("{:b}", x + y);
    let mut orig_z: String = "".to_string();

    for o in &z {
        let res: bool;
        let t = calc(constants.clone(), o.to_string(), &gates, 1);
        match t {
            Ok((c, b)) => {
                constants = c;
                res = b;
            }
            Err(s) => break,
        };
        if res {
            orig_z.push('1');
        } else {
            orig_z.push('0');
        }
    }

    let differences: Vec<usize> = goal_z
        .chars()
        .zip(orig_z.chars()) // Pair characters from both strings
        .enumerate() // Add indices to the pairs
        .filter_map(|(i, (c1, c2))| if c1 != c2 { Some(i) } else { None }) // Filter differences
        .collect();

    println!("{:?}", differences);
    let z_with_diff: Vec<&str> = vec!["z00", "z02", "z03", "z04", "z05", "z14", "z17", "z18", "z19", "z20", "z22"];

    let mut keys: Vec<String> = Vec::new();

    for z in z_with_diff {
        keys = add_keys(keys, z.to_string(), &gates);
    }

    println!("keys length: {}", keys.len());

    let mut loop_var = false;

    let mut j = 0;
    let mut tried: HashSet<(&str, &str, &str, &str, &str, &str, &str, &str)> = HashSet::new();
    // let keys: Vec<&str> = gates.keys().copied().collect();
    let mut k1: &str = "";
    let mut k2: &str = "";
    let mut k3: &str = "";
    let mut k4: &str = "";
    let mut k5: &str = "";
    let mut k6: &str = "";
    let mut k7: &str = "";
    let mut k8: &str = "";
    println!("{}", y + x);
    while !loop_var {
        if j % 5000 == 0 {
            println!("{}", j);
        }

        //mutate gates
        let mut tmp: HashMap<&str, (&str, &str, usize)> = gates.clone();

        k1 = keys.choose(&mut rand::thread_rng()).unwrap();
        k2 = keys.choose(&mut rand::thread_rng()).unwrap();
        k3 = keys.choose(&mut rand::thread_rng()).unwrap();
        k4 = keys.choose(&mut rand::thread_rng()).unwrap();
        k5 = keys.choose(&mut rand::thread_rng()).unwrap();
        k6 = keys.choose(&mut rand::thread_rng()).unwrap();
        k7 = keys.choose(&mut rand::thread_rng()).unwrap();
        k8 = keys.choose(&mut rand::thread_rng()).unwrap();

        while tried.contains(&(k1, k2, k3, k4, k5, k6, k7, k8)) {
            let k1 = keys.choose(&mut rand::thread_rng()).unwrap();
            let k2 = keys.choose(&mut rand::thread_rng()).unwrap();
            let k3 = keys.choose(&mut rand::thread_rng()).unwrap();
            let k4 = keys.choose(&mut rand::thread_rng()).unwrap();
            let k5 = keys.choose(&mut rand::thread_rng()).unwrap();
            let k6 = keys.choose(&mut rand::thread_rng()).unwrap();
            let k7 = keys.choose(&mut rand::thread_rng()).unwrap();
            let k8 = keys.choose(&mut rand::thread_rng()).unwrap();
        }

        tried.insert((k1, k2, k3, k4, k5, k6, k7, k8));

        let t1 = *tmp.get(k1).unwrap();
        let t2 = *tmp.get(k2).unwrap();
        let t3 = *tmp.get(k3).unwrap();
        let t4 = *tmp.get(k4).unwrap();
        let t5 = *tmp.get(k5).unwrap();
        let t6 = *tmp.get(k6).unwrap();
        let t7 = *tmp.get(k6).unwrap();
        let t8 = *tmp.get(k6).unwrap();

        tmp.insert(k1, t2);
        tmp.insert(k2, t1);
        tmp.insert(k3, t4);
        tmp.insert(k4, t3);
        tmp.insert(k5, t6);
        tmp.insert(k6, t5);
        tmp.insert(k7, t8);
        tmp.insert(k8, t7);

        //end mutate
        loop_var = calc_part2(&z, x + y, &tmp, constants.clone());

        j += 1;
    }

    println!(
        "{}, {}, {}, {}, {}, {}, {}, {}",
        k1, k2, k3, k4, k5, k6, k7, k8
    );
}

fn calc_part2(
    z: &Vec<&str>,
    goal: u64,
    gates: &HashMap<&str, (&str, &str, usize)>,
    mut constants: HashMap<String, bool>,
) -> bool {
    let mut num: Vec<bool> = Vec::new();

    for o in z {
        let res: bool;
        let t = calc(constants, o.to_string(), gates, 1);
        match t {
            Ok((c, b)) => {
                constants = c;
                res = b;
            }
            Err(s) => break,
        };

        num.push(res);
    }
    if z.len() != 46 {
        false
    } else {
        let z = u64::from_str_radix(&num.iter().map(|x| *x as usize).rev().join(""), 2);
        match z {
            Ok(t) => t == goal,
            Err(_) => false,
        }
    }
}

fn add_keys(
    mut keys: Vec<String>,
    input: String,
    gates: &HashMap<&str, (&str, &str, usize)>,
) -> Vec<String> {
    if keys.contains(&input.clone()) || input.starts_with("x") || input.starts_with("y"){
        keys
    } else {
        keys.push(input.clone());
        let gate = gates.get(&input[..]).unwrap();
        keys = add_keys(keys, gate.0.to_string(), gates);
        keys = add_keys(keys, gate.1.to_string(), gates);

        keys
    }
}
