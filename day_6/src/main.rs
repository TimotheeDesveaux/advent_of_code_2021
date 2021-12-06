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

fn main() {
    let input = include_str!("example.txt");

    let number_of_fish = part_one(input);

    println!("The number of fish after 80 days is {}", number_of_fish)
}
