fn part_one(input: &str) -> u32 {
    let mut heights: Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {
        let line: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        heights.push(line);
    }

    let mut count = 0;
    for y in 0..heights.len() {
        for x in 0..heights[y].len() {
            let height = heights[y][x];
            if (y == 0 || heights[y - 1][x] > height)
                && (x == heights[y].len() - 1 || heights[y][x + 1] > height)
                && (y == heights.len() - 1 || heights[y + 1][x] > height)
                && (x == 0 || heights[y][x - 1] > height)
            {
                count += heights[y][x] + 1;
            }
        }
    }

    count
}

fn main() {
    let input = include_str!("input.txt");

    let risk_level = part_one(input);
    println!(
        "The sum of the risk levels of all low poiiints is {}",
        risk_level
    );
}
