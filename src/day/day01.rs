use std::{
    fs,
    time::{Duration, Instant},
};

#[derive(Debug)]
enum Dir {
    L,
    R,
}
impl Dir {
    fn new(s: char) -> Self {
        match s {
            'L' => Dir::L,
            'R' => Dir::R,
            _ => panic!("Invalid Direction"),
        }
    }
}

fn parse_input(input: &str) -> Vec<(Dir, i32)> {
    input
        .lines()
        .into_iter()
        .map(|v| {
            let c: Vec<char> = v.chars().into_iter().collect();
            (match c.as_slice() {
                [first, d1] => {
                    let num = d1.to_digit(10).unwrap() as i32;
                    Some((Dir::new(*first), num))
                }
                [first, d1, d2] => {
                    let s: String = [*d1, *d2].iter().collect();
                    let num = s.parse::<i32>().unwrap();
                    Some((Dir::new(*first), num))
                }
                [first, d1, d2, d3] => {
                    let s: String = [*d1, *d2, *d3].iter().collect();
                    let num = s.parse::<i32>().unwrap();
                    Some((Dir::new(*first), num))
                }
                _ => None,
            })
            .unwrap()
        })
        .collect()
}

pub fn solve_part1(input: &str) -> u128 {
    let puzzle = parse_input(input);
    let mut dial = 50;
    let mut zero_count = 0;
    for (dir, amount) in puzzle {
        match dir {
            Dir::L => dial = ((dial - amount) + 1000) % 100,
            Dir::R => dial = ((dial + amount) + 1000) % 100,
        }
        if dial == 0 {
            zero_count += 1;
        }
    }
    zero_count
}

fn click(dial: i32, dir: &Dir) -> i32 {
    let mut new_dial: i32;
    match dir {
        Dir::L => {
            new_dial = dial - 1;
            if new_dial < 0 {
                new_dial += 100;
            }
        }
        Dir::R => {
            new_dial = dial + 1;
            if new_dial > 99 {
                new_dial -= 100;
            }
        }
    }
    new_dial
}
pub fn solve_part2(input: &str) -> u128 {
    let puzzle = parse_input(input);
    let mut dial = 50;
    let mut zero_count = 0;
    for (dir, amount) in puzzle {
        for _ in 0..amount {
            dial = click(dial, &dir);
            if dial == 0 {
                zero_count += 1;
            }
        }
    }
    zero_count
}

pub fn solve_from_file(path: &str) -> (u128, Duration, u128, Duration) {
    let data = fs::read_to_string(path).expect("failed to read input");

    let p1start = Instant::now();
    let p1answer = solve_part1(&data);
    let p1time = p1start.elapsed();

    let p2start = Instant::now();
    let p2answer = solve_part2(&data);
    let p2time = p2start.elapsed();

    (p1answer, p1time, p2answer, p2time)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(solve_part1(input), 3);
        assert_eq!(solve_part2(input), 6);
    }
}
