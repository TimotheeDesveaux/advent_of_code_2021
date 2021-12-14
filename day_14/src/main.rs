use itertools::Itertools;
use std::collections::HashMap;

fn part_one(input: &str) -> u32 {
    let (template, pairs_insertion) = input.split_once("\n\n").unwrap();

    let mut pairs: Vec<(char, char)> = template.chars().tuple_windows().collect();

    let pairs_insertion: HashMap<(char, char), char> = pairs_insertion
        .lines()
        .map(|l| l.split_once(" -> ").unwrap())
        .map(|(p, i)| {
            (
                p.chars().collect_tuple().unwrap(),
                i.chars().exactly_one().unwrap(),
            )
        })
        .collect();

    for _ in 0..10 {
        let mut pairs_tmp: Vec<(char, char)> = Vec::new();
        for pair in pairs {
            let insert = pairs_insertion[&pair];
            pairs_tmp.push((pair.0, insert));
            pairs_tmp.push((insert, pair.1));
        }
        pairs = pairs_tmp;
    }

    let mut hist = [0; 26];
    for pair in &pairs {
        hist[pair.0 as usize - 'A' as usize] += 1;
    }
    hist[pairs.last().unwrap().1 as usize - 'A' as usize] += 1;

    hist.iter().max().unwrap() - hist.iter().filter(|&&n| n > 0).min().unwrap()
}

fn main() {
    let input = include_str!("input.txt");

    let sub = part_one(input);

    println!(
        "The difference between the occurence of the most common element and the least common element is {}",
         sub);
}
