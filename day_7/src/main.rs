fn part_one(input: &str) -> i32 {
    let positions: Vec<i32> = input
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect();

    let min = positions.iter().min().unwrap();
    let max = positions.iter().max().unwrap();

    let mut min_fuel = -1;
    for position in (*min)..=(*max) {
        let fuel = positions
            .iter()
            .fold(0, |acc, p| acc + i32::abs(p - position));

        if min_fuel == -1 || fuel < min_fuel {
            min_fuel = fuel;
        }
    }

    min_fuel
}

fn part_two(input: &str) -> i32 {
    let positions: Vec<i32> = input
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect();

    let min = positions.iter().min().unwrap();
    let max = positions.iter().max().unwrap();

    let mut min_fuel = -1;
    for position in (*min)..=(*max) {
        let fuel = positions.iter().fold(0, |acc, p| {
            let distance = i32::abs(p - position);
            acc + (distance * (distance + 1)) / 2 // Sum of first integers
        });

        if min_fuel == -1 || fuel < min_fuel {
            min_fuel = fuel;
        }
    }

    min_fuel
}

fn main() {
    let input = include_str!("input.txt");

    let min_fuel1 = part_one(input);
    let min_fuel2 = part_two(input);

    println!(
        "The minimum fuel needed for the crabs to align is {}",
        min_fuel1
    );
    println!(
        "The corrected minimum fuel needed for the crabs to align is {}",
        min_fuel2
    );
}
