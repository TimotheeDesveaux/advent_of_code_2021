use std::ops::RangeInclusive;

#[macro_use]
extern crate scan_fmt;

fn try_velocity(
    mut vx: i32,
    mut vy: i32,
    area_x: RangeInclusive<i32>,
    area_y: RangeInclusive<i32>,
) -> Option<i32> {
    let (mut x, mut y) = (0, 0);
    let mut max_y = 0;

    loop {
        x += vx;
        y += vy;
        vx -= i32::signum(vx);
        vy -= 1;
        max_y = i32::max(max_y, y);

        return match (area_x.contains(&x), area_y.contains(&y)) {
            (true, true) => Some(max_y),
            (false, _) if vx == 0 => None,
            (_, false) if vy < 0 && &y < area_y.end() => None,
            _ => continue,
        };
    }
}

fn part_one(input: &str) -> i32 {
    let (min_area_x, max_area_x, min_area_y, max_area_y) =
        scan_fmt!(input, "target area: x={}..{}, y={}..{}", i32, i32, i32, i32).unwrap();

    let mut max_y = 0;
    for vy in min_area_y..1000 {
        for vx in 0..max_area_x {
            if let Some(y) = try_velocity(vx, vy, min_area_x..=max_area_x, min_area_y..=max_area_y)
            {
                max_y = i32::max(max_y, y);
            }
        }
    }

    max_y
}

fn part_two(input: &str) -> i32 {
    let (min_area_x, max_area_x, min_area_y, max_area_y) =
        scan_fmt!(input, "target area: x={}..{}, y={}..{}", i32, i32, i32, i32).unwrap();

    let mut count = 0;
    for vy in min_area_y..1000 {
        for vx in 0..=max_area_x {
            if let Some(_) = try_velocity(vx, vy, min_area_x..=max_area_x, min_area_y..=max_area_y)
            {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let input = include_str!("input.txt");

    let max_y = part_one(input);
    let count = part_two(input);

    println!("The highest y position reached is {}", max_y);
    println!(
        "The number of velocity values that cause the probe to hit the target is {}",
        count
    );
}
