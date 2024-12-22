use std::collections::{HashMap, HashSet, VecDeque};


pub fn part1(lines: Vec<String>) {
    let mut sum = 0;
    for line in lines {
        let number: i64 = line.parse().unwrap();
        let mut tmp3 = number;
        for _ in 0..2000 {
            let tmp1 = handle(tmp3, tmp3 << 6);
            let tmp2 = handle(tmp1, tmp1 >> 5);
            tmp3 = handle(tmp2, tmp2 << 11);
        }
        println!("{}: {}", number, tmp3);
        sum += tmp3;
    }
    println!("{}", sum);
}

fn handle(secret: i64, given: i64) -> i64 {
    let tmp = secret ^ given;
    tmp & 16777215
}

pub fn part2(lines: Vec<String>) {
    let start_values: Vec<i64> = lines.iter().map(|x| x.parse().unwrap()).collect();
    let mut result: HashMap<VecDeque<i64>, i64> = HashMap::new();
    for number in &start_values {
        let mut tmp3 = *number;
        let mut last_price = tmp3 % 10;
        let mut seq: VecDeque<i64> = VecDeque::new();
        let mut handled_pats = HashSet::new();
        for _ in 0..2000 {
            let tmp1 = handle(tmp3, tmp3 << 6);
            let tmp2 = handle(tmp1, tmp1 >> 5);
            tmp3 = handle(tmp2, tmp2 << 11);
            let price: i64 = tmp3 % 10;
            let diff = price - last_price;
            seq.push_back(diff);
            if !handled_pats.contains(&seq){
                *result.entry(seq.clone()).or_insert(0) += price;
                handled_pats.insert(seq.clone());
            }
            if seq.len() == 4{
                seq.pop_front();
            }
            last_price = price;
        }
    }
    let (best_pat, max) = result.iter().max_by_key(|&(_, &val)| val).unwrap();


    println!("{} {:?}", max, best_pat);
}


