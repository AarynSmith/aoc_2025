use std::{
    fs,
    time::{Duration, Instant},
};

fn parse_input(input: &str) -> Vec<(u128, u128)> {
    input
        .split(",")
        .map(|part| {
            let mut pieces = part.split('-');
            let start = pieces.next().unwrap().parse::<u128>().unwrap();
            let end = pieces.next().unwrap().parse::<u128>().unwrap();
            (start, end)
        })
        .collect()
}

fn is_silly(n: u128) -> bool {
    let n_string = format!("{}", n).to_string();
    if n_string.len() % 2 == 1 {
        return false;
    }
    let mid = n_string.len() / 2;
    let (left, right) = n_string.split_at(mid);
    left == right
}

pub fn solve_part1(input: &str) -> u128 {
    let puzzle = parse_input(input);
    let mut silly_sum = 0;
    for (start, end) in puzzle {
        for n in start..=end {
            if is_silly(n) {
                silly_sum += n;
            }
        }
    }
    silly_sum
}

fn is_extra_silly(n: u128) -> bool {
    if is_silly(n) {
        return true;
    }
    let n_string = format!("{}", n).to_string();
    for chunk in 1..=n_string.len() / 2 {
        let (left, _right) = n_string.split_at(chunk);
        if n_string.len() % left.len() != 0 {
            continue;
        }
        if left.repeat(n_string.len() / left.len()) == n_string {
            return true;
        }
    }
    return false;
}

pub fn solve_part2(input: &str) -> u128 {
    let puzzle = parse_input(input);
    let mut silly_sum = 0;
    for (start, end) in puzzle {
        for n in start..=end {
            if is_extra_silly(n) {
                silly_sum += n;
            }
        }
    }
    silly_sum
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
    fn is_silly_test() {
        let tests = vec![
            (11, true),
            (10, false),
            (22, true),
            (1010, true),
            (1188511885, true),
            (55, true),
            (6464, true),
            (123123, true),
            (101, false),
            (1, false),
            (0, false),
            (100, false),
            (100100, true),
            (100100100, false),
        ];
        for test in tests {
            assert_eq!(is_silly(test.0), test.1);
        }
    }

    #[test]
    fn is_extra_silly_test() {
        let tests = vec![
            (11, true),
            (10, false),
            (22, true),
            (222, true),
            (1010, true),
            (1188511885, true),
            (55, true),
            (6464, true),
            (123123, true),
            (101, false),
            (1, false),
            (0, false),
            (100, false),
            (100100, true),
            (100100100, true),
            (999, true),
            (565656, true),
            (2121212121, true),
        ];
        for (test, expected) in tests {
            let got = is_extra_silly(test);
            assert_eq!(
                got, expected,
                "Test: {} failed, expected: {}, got: {}",
                test, expected, got
            );
        }
    }

    #[test]
    fn example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(solve_part1(input), 1227775554);
        assert_eq!(solve_part2(input), 4174379265);
    }
}
