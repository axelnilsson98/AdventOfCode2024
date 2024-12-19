use std::collections::HashMap;

pub fn part1(lines: Vec<String>){
    let towels: Vec<&str> = lines[0].split(", ").collect();
    println!("{:?}", towels);

    let mut nbr_valid = 0;
    for i in 2..lines.len(){
        if valid(&towels, &lines[i][0..]){
            nbr_valid += 1;
        }
    }
    println!("{}", nbr_valid);
}

fn valid(towels: &Vec<&str>, remaining: &str) -> bool{
    if remaining.len() == 0{
        return true;
    }
    for towel in towels{
        if remaining.starts_with(towel) && valid(towels, &remaining[towel.len()..]){
            return true;
        }
    }
    
    false
}

pub fn part2(lines: Vec<String>){
    let mut map: HashMap<String, i64> = HashMap::new();
    let towels: Vec<&str> = lines[0].split(", ").collect();
    println!("{:?}", towels);

    let mut res = 0;
    for i in 2..lines.len(){
        let tmp: i64;
        (map, tmp) = nbr_valid(&towels, &lines[i][0..], map);
        res += tmp;
    }
    println!("{}", res);
}


fn nbr_valid(towels: &Vec<&str>, remaining: &str, mut map: HashMap<String, i64>) -> (HashMap<String, i64>, i64){
    if map.contains_key(remaining){
        let tmp = *map.get(&remaining.to_string()).unwrap();
        return (map, tmp);
    }
    if remaining.len() == 0{
        return (map, 1);
    }
    let mut nbr = 0;
    for towel in towels{
        if remaining.starts_with(towel){
            let tmp: i64;
            (map, tmp) = nbr_valid(towels, &remaining[towel.len()..], map);
            nbr += tmp;
        }
    }
    map.insert(remaining.to_string(), nbr);
    (map, nbr)
}
