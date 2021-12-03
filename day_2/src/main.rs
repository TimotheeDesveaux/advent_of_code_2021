fn part_one(input: &str) -> i32 {
    let mut depth = 0;
    let mut horizontal_pos = 0;

    for line in input.lines() {
        let sline: Vec<&str> = line.split(' ').collect();
        let direction = sline[0];
        let amount: i32 = sline[1].trim().parse().unwrap();

        match direction {
            "forward" => horizontal_pos += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => (),
        }
    }

    depth * horizontal_pos
}

fn part_two(input: &str) -> i32 {
    let mut depth = 0;
    let mut horizontal_pos = 0;
    let mut aim = 0;

    for line in input.lines() {
        let sline: Vec<&str> = line.split(' ').collect();
        let direction = sline[0];
        let amount: i32 = sline[1].trim().parse().unwrap();

        match direction {
            "forward" => {
                horizontal_pos += amount;
                depth += aim * amount
            }
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => (),
        }
    }

    depth * horizontal_pos
}

fn main() {
    let input = include_str!("input.txt");
    let position1 = part_one(input);
    let position2 = part_two(input);

    println!(
        "The position after following the planned course is {}",
        position1
    );

    println!(
        "The position after following the corrected planned course is {}",
        position2
    );
}
