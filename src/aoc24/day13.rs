use nalgebra::{Matrix2, Vector2};

pub fn part1(lines: Vec<String>) {
    let epsilon: f32 = 0.0001;
    let no_blanks: Vec<&String> = lines.iter().filter(|line| *line != "").collect();
    let mut sum = 0;
    let mut a: (f32, f32) = (0.0, 0.0);
    let mut b: (f32, f32) = (0.0, 0.0);
    let mut prize: (f32, f32);
    for (i, str) in no_blanks.iter().enumerate() {
        // println!("{}", i);
        let nums: Vec<f32> = str
            .split(|c: char| !c.is_numeric())
            .filter(|x| *x != "")
            .map(|x| x.parse::<f32>().unwrap())
            .collect();
        if i % 3 == 0 {
            a = (nums[0], nums[1]);
        }
        if i % 3 == 1 {
            b = (nums[0], nums[1]);
        }
        if i % 3 == 2 {
            prize = (nums[0], nums[1]);

            let transformation_matrix = Matrix2::new(a.0, b.0, a.1, b.1);
            let v = Vector2::new(prize.0, prize.1);

            let transformation_matrix_inv = transformation_matrix.try_inverse().unwrap();
            let result = transformation_matrix_inv * v;

            let a: i32 = result[0].round() as i32;
            let b: i32 = result[1].round() as i32;
            if (a as f32 -result[0]).abs() < epsilon && (b as f32-result[1]).abs() < epsilon{
                sum += 3*a + b;
            } 

        }
    }
    println!("{}", sum);
}


pub fn part2(lines: Vec<String>) {
    let epsilon: f64 = 0.01;
    let no_blanks: Vec<&String> = lines.iter().filter(|line| *line != "").collect();
    let mut sum = 0;
    let mut a: (f64, f64) = (0.0, 0.0);
    let mut b: (f64, f64) = (0.0, 0.0);
    let mut prize: (f64, f64);
    for (i, str) in no_blanks.iter().enumerate() {
        let nums: Vec<f64> = str
            .split(|c: char| !c.is_numeric())
            .filter(|x| *x != "")
            .map(|x| x.parse::<f64>().unwrap())
            .collect();
        if i % 3 == 0 {
            a = (nums[0], nums[1]);
        }
        if i % 3 == 1 {
            b = (nums[0], nums[1]);
        }
        if i % 3 == 2 {
            prize = (nums[0] + 10_000_000_000_000.0, nums[1] + 10_000_000_000_000.0);

            let transformation_matrix: nalgebra::Matrix<f64, nalgebra::Const<2>, nalgebra::Const<2>, nalgebra::ArrayStorage<f64, 2, 2>> = Matrix2::new(a.0, b.0, a.1, b.1);
            let v = Vector2::new(prize.0, prize.1);

            let transformation_matrix_inv = transformation_matrix.try_inverse().unwrap();
            let result = transformation_matrix_inv * v;

            let a: i64 = result[0].round() as i64;
            let b: i64 = result[1].round() as i64;
            if (a as f64 -result[0]).abs() < epsilon && (b as f64-result[1]).abs() < epsilon{
                sum += 3*a + b;
            } 

        }
    }
    println!("{}", sum);
}

