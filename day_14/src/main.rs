use itertools::Itertools;
use std::collections::HashMap;

fn solve(input: &str, step: u32) -> u64 {
    let (template, pairs_insertion) = input.split_once("\n\n").unwrap();

    let pair_insertions: HashMap<(char, char), char> = pairs_insertion
        .lines()
        .map(|l| l.split_once(" -> ").unwrap())
        .map(|(p, i)| {
            (
                p.chars().collect_tuple().unwrap(),
                i.chars().exactly_one().unwrap(),
            )
        })
        .collect();

    let mut pair_frequencies: HashMap<(char, char), u64> =
        template.chars().tuple_windows().map(|p| (p, 1)).collect();

    for _ in 0..step {
        let mut pair_frequencies_tmp: HashMap<(char, char), u64> = HashMap::new();
        for (pair, freq) in &pair_frequencies {
            let insert = pair_insertions[pair];
            *pair_frequencies_tmp.entry((pair.0, insert)).or_default() += freq;
            *pair_frequencies_tmp.entry((insert, pair.1)).or_default() += freq;
        }

        pair_frequencies = pair_frequencies_tmp;
    }

    let mut hist = [0; 26];
    for (pair, freq) in &pair_frequencies {
        hist[pair.0 as usize - 'A' as usize] += freq;
    }
    hist[template.chars().last().unwrap() as usize - 'A' as usize] += 1;

    hist.iter().max().unwrap() - hist.iter().filter(|&&n| n > 0).min().unwrap()
}

fn part_one(input: &str) -> u64 {
    solve(input, 10)
}

fn part_two(input: &str) -> u64 {
    solve(input, 40)
}

fn main() {
    let input = include_str!("input.txt");

    let sub1 = part_one(input);
    let sub2 = part_two(input);

    println!(
        "The difference between the occurence of the most common element and the least common element after 10 steps is {}",
         sub1);
    println!(
        "The difference between the occurence of the most common element and the least common element after 40 steps is {}",
         sub2);
}
