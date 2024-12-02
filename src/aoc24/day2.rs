pub fn part1(lines: Vec<String>){
    let mut num = 0;
    for line in &lines{
        
        let v: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let safe = is_safe(&v);
        
        if safe {
            num += 1;
        }
    }

    println!("{}", num);
}


pub fn part2(lines: Vec<String>) {
    let mut num = 0;

    for line in &lines {
        let levels: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();

        // Check if the report is already safe
        if is_safe(&levels) {
            num += 1;
            continue;
        }

        // Try removing one level at a time
        let mut found_safe = false;
        for i in 0..levels.len() {
            let mut modified = levels.clone();
            modified.remove(i);

            if is_safe(&modified) {
                found_safe = true;
                break;
            }
        }

        if found_safe {
            num += 1;
        }
    }

    println!("{}", num);
}

fn is_safe(v: &[i32]) -> bool {
    if v.len() < 2 {
        return true; // Single or empty reports are trivially safe
    }

    let incr = v[0] > v[1];

    for i in 0..v.len()-1{
        let t1:i32 = v[i]; 
        let t2:i32 = v[i+1]; 
        let diff = (t1 - t2).abs();
        if diff == 0 || diff > 3{
            return false;
        }
        if incr {
            if t1 < t2{
                return false;
            }
        }else{
            if t1 > t2{
                return false;
            }        
        }
        
    }
    true

}