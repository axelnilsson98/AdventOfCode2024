static ILLEGAL_ARROW: (usize, usize) = (0, 0);
static UP: (usize, usize) = (0, 1);
static A_ARROW: (usize, usize) = (0, 2);
static DOWN: (usize, usize) = (1, 0);
static RIGHT: (usize, usize) = (1, 1);
static LEFT: (usize, usize) = (1, 2);

use std::collections::VecDeque;

static SEVEN: (usize, usize) = (0, 0);
static EIGHT: (usize, usize) = (0, 1);
static NINE: (usize, usize) = (0, 2);
static FOUR: (usize, usize) = (1, 0);
static FIVE: (usize, usize) = (1, 1);
static SIX: (usize, usize) = (1, 2);
static ONE: (usize, usize) = (2, 0);
static TWO: (usize, usize) = (2, 1);
static THREE: (usize, usize) = (2, 2);
static ILLEGAL_NUM: (usize, usize) = (3, 0);
static ZERO: (usize, usize) = (3, 1);
static A_NUM: (usize, usize) = (3, 2);

// static ILLEGAL_ARROW: &str = "ILL";
// static UP: &str = "UP";
// static A_ARROW: &str = "A";
// static DOWN: &str = "DOWN";
// static RIGHT:&str = "RIGHT";
// static LEFT: &str = "LEFT";

pub fn part1(lines: Vec<String>) {
    for line in lines {
        let mut code: Vec<(usize, usize)> = line
            .chars()
            .map(|x| match x {
                '0' => ZERO,
                '1' => ONE,
                '2' => TWO,
                '3' => THREE,
                '4' => FOUR,
                '5' => FIVE,
                '6' => SIX,
                '7' => SEVEN,
                '8' => EIGHT,
                '9' => NINE,
                'A' => A_NUM,
                _ => panic!("illegal number"),
            })
            .collect();
        let mut pos = A_NUM;
        for _ in 0..3 {
            let mut next_seq: Vec<(usize, usize)> = Vec::new();

            for end in &code {
                next_seq.extend(find_sequence(pos, *end, ILLEGAL_NUM));
                pos = *end;
            }
            println!("{}", next_seq.len());
            code = next_seq.clone();
        }
    }
}

fn find_sequence(
    start: (usize, usize),
    end: (usize, usize),
    illegal: (usize, usize),
) -> VecDeque<(usize, usize)> {
    let mut res = VecDeque::new();
    if start.1 == illegal.1 {
        res.push_back(RIGHT);
    }
    if start.0 < end.0 {
        for _ in 0..(end.0 - start.0) {
            res.push_back(DOWN);
        }
    } else if start.0 > end.0 {
        for _ in 0..(start.0 - end.0) {
            res.push_back(UP);
        }
    }

    if start.1 < end.1 {
        for _ in 0..(end.1 - start.1) {
            res.push_back(RIGHT);
        }
    } else if start.1 > end.1 {
        for _ in 0..(start.1 - end.1) {
            res.push_back(LEFT);
        }
    }

    res.push_back(A_ARROW);
    res
}
