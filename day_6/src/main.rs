fn part_one(input: &str) -> usize {
    let mut fish: Vec<u32> = input
        .split(',')
        .map(|f| f.trim().parse().unwrap())
        .collect();

    for _ in 0..80 {
        let mut new_fish = 0;
        fish = fish
            .iter()
            .map(|&f| {
                if f == 0 {
                    new_fish += 1;
                    6
                } else {
                    f - 1
                }
            })
            .collect();
        fish.append(&mut vec![8; new_fish]);
    }

    fish.len()
}

fn part_two(input: &str) -> usize {
    let mut frequencies = input
        .split(',')
        .map(|f| f.trim().parse::<usize>().unwrap())
        .fold([0; 9], |mut acc, n| {
            acc[n] += 1;
            acc
        });

    for _ in 0..256 {
        frequencies.rotate_left(1);
        frequencies[6] += frequencies[8];
    }

    frequencies.iter().sum()
}

fn main() {
    let input = include_str!("input.txt");

    let number_of_fish1 = part_one(input);
    let number_of_fish2 = part_two(input);

    println!("The number of fish after 80 days is {}", number_of_fish1);
    println!("The number of fish after 256 days is {}", number_of_fish2);
}
