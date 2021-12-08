use std::collections::HashSet;

fn part_one(input: &str) -> u32 {
    let mut res = 0;
    for line in input.lines() {
        let digits = line.split(" | ").nth(1).unwrap().split_whitespace();
        res += digits.fold(0, |acc, d| {
            let len = d.len();
            if len == 2 || len == 3 || len == 4 || len == 7 {
                acc + 1
            } else {
                acc
            }
        });
    }

    res
}

fn part_two(input: &str) -> u32 {
    let mut res = 0;
    for line in input.lines() {
        let mut sline = line.split(" | ");

        let signals: Vec<HashSet<char>> = sline
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.chars().collect::<HashSet<char>>())
            .collect();
        let one = signals.iter().position(|s| s.len() == 2).unwrap();
        let four = signals.iter().position(|s| s.len() == 4).unwrap();

        let output: Vec<HashSet<char>> = sline
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.chars().collect::<HashSet<char>>())
            .collect();

        let mut entry = 0;
        for digit in output {
            let digit = match (
                digit.len(),
                digit.intersection(&signals[four]).count(),
                digit.intersection(&signals[one]).count(),
            ) {
                (6, 3, 2) => 0,
                (2, _, _) => 1,
                (5, 2, _) => 2,
                (5, 3, 2) => 3,
                (4, _, _) => 4,
                (5, 3, 1) => 5,
                (6, 3, 1) => 6,
                (3, _, _) => 7,
                (7, _, _) => 8,
                (6, 4, _) => 9,
                _ => panic!(),
            };

            entry = entry * 10 + digit;
        }

        res += entry;
    }

    res
}

fn main() {
    let input = include_str!("input.txt");

    let count1 = part_one(input);
    let count2 = part_two(input);

    println!("1, 4, 7 or 8 appear {} times", count1);
    println!("The sum of all outputs is {}", count2);
}
