use std::collections::HashSet;

fn get_lower_points(heights: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut lower_points = Vec::new();
    for y in 0..heights.len() {
        for x in 0..heights[y].len() {
            let height = heights[y][x];
            if (y == 0 || heights[y - 1][x] > height)
                && (x == heights[y].len() - 1 || heights[y][x + 1] > height)
                && (y == heights.len() - 1 || heights[y + 1][x] > height)
                && (x == 0 || heights[y][x - 1] > height)
            {
                lower_points.push((y, x));
            }
        }
    }

    lower_points
}

fn basin_size(
    heights: &Vec<Vec<u32>>,
    seen: &mut HashSet<(usize, usize)>,
    point: (usize, usize),
) -> u32 {
    let height = heights[point.0][point.1];
    let mut size = 1;

    if !seen.insert(point) || height == 9 {
        return 0;
    }

    if point.0 > 0 {
        size += basin_size(heights, seen, (point.0 - 1, point.1))
    }
    if point.1 < heights[point.0].len() - 1 {
        size += basin_size(heights, seen, (point.0, point.1 + 1))
    }
    if point.1 > 0 {
        size += basin_size(heights, seen, (point.0, point.1 - 1))
    }
    if point.0 < heights.len() - 1 {
        size += basin_size(heights, seen, (point.0 + 1, point.1))
    }

    size
}

fn part_one(input: &str) -> u32 {
    let mut heights: Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {
        let line: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        heights.push(line);
    }

    get_lower_points(&heights)
        .iter()
        .fold(0, |acc, p| acc + heights[p.0][p.1] + 1)
}

fn part_two(input: &str) -> u32 {
    let mut heights: Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {
        let line: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        heights.push(line);
    }

    let mut seen = HashSet::new();
    let mut sizes: Vec<u32> = get_lower_points(&heights)
        .iter()
        .map(|p| basin_size(&heights, &mut seen, *p))
        .collect();

    sizes.sort();
    sizes.iter().rev().take(3).product()
}

fn main() {
    let input = include_str!("input.txt");

    let risk_level = part_one(input);
    let size = part_two(input);

    println!(
        "The sum of the risk levels of all lower points is {}",
        risk_level
    );
    println!(
        "The product of the size of the three bigger bain is {}",
        size
    );
}
