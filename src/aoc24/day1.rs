pub fn part1(lines: Vec<String>){
    let mut list1: Vec<i64> = vec![];
    let mut list2: Vec<i64> = vec![];
    // let mut sum: i64 = 0;

    // Print the lines
    for line in &lines {
        let v: Vec<i64> = line.split("   ").map(|x| x.parse().unwrap()).collect();
        list1.push(v[0]);
        list2.push(v[1]);
    }
    list1.sort_unstable();
    list2.sort_unstable();

    let sum: i64 = list1
        .iter()
        .zip(list2)
        .map(|(a,b)| (a-b).abs())
        .sum();


    println!("{}", sum);
}

pub fn part2(lines: Vec<String>){
    let mut list1: Vec<usize> = vec![];
    let mut list2: Vec<usize> = vec![];
    // let mut sum: usize = 0;

    // Print the lines
    for line in &lines {
        let v: Vec<usize> = line.split("   ").map(|x| x.parse().unwrap()).collect();
        list1.push(v[0]);
        list2.push(v[1]);
    }
    list1.sort_unstable();
    list2.sort_unstable();

    let sum: usize = list1
        .iter()
        .map(|x| x * list2
            .iter()
            .filter(|&&y| y == *x)
            .count())
        .sum();

    println!("{}", sum);
}