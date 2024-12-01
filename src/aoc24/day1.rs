pub fn part1(lines: Vec<String>){
    let mut list1: Vec<i64> = vec![];
    let mut list2: Vec<i64> = vec![];
    let mut sum: i64 = 0;

    // Print the lines
    for line in &lines {
        let v: Vec<&str> = line.split("   ").collect();
        list1.push(v[0].parse().unwrap());
        list2.push(v[1].parse().unwrap());
    }
    list1.sort();
    list2.sort();

    for i in 0..list1.len(){
        if list1[i] > list2[i]{
            sum = sum + (list1[i] - list2[i]);
        }
        else{
            sum = sum + (list2[i] - list1[i]);
        }
    }

    println!("{}", sum);
}

pub fn part2(lines: Vec<String>){
    let mut list1: Vec<usize> = vec![];
    let mut list2: Vec<usize> = vec![];
    let mut sum: usize = 0;

    // Print the lines
    for line in &lines {
        let v: Vec<&str> = line.split("   ").collect();
        list1.push(v[0].parse().unwrap());
        list2.push(v[1].parse().unwrap());
    }
    list1.sort();
    list2.sort();

    for i in 0..list1.len(){
        let count = list2.iter().filter(|&&x| x == list1[i]).count();

        sum = sum + (list1[i] * count);
    }
    println!("{}", sum);
}