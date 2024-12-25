pub fn part1(lines: Vec<String>){
    let mut keys: Vec<Vec<i32>> = Vec::new();
    let mut holes: Vec<Vec<i32>> = Vec::new();

    for i in (0..lines.len()).step_by(8){
        if lines[i] == "#####"{
            holes = parse(&lines, i, holes);
        }else{
            keys = parse(&lines, i, keys);
        }

    }
    println!("keys: {:?}", keys);
    println!("holes: {:?}", holes);
    let mut res = 0;

    for key in keys{
        for hole in &holes{
            let mut possible = true;
            key.iter().zip(hole).for_each(|(k,h)| if h + k > 5{
                possible = false;
            });
            if possible {
                res += 1;
            }
        }
    }
    println!("{}", res);
}

fn parse(lines: &Vec<String>, i: usize, mut vec: Vec<Vec<i32>>) -> Vec<Vec<i32>>{
    let mut tmp = vec![0;5];
    for j in 1..6{
        lines[i+j].chars().enumerate().for_each(|(k, c)| {
            if c == '#'{
                tmp[k] +=1;
            }
        });
    }
    vec.push(tmp);
    vec
}