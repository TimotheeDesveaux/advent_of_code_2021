use std::collections::HashMap;

fn register_vent(
    start: &(u32, u32),
    end: &(u32, u32),
    points_covered: &mut HashMap<(u32, u32), u32>,
) {
    if start.0 == end.0 {
        let x = start.0;
        let mut start_y = start.1;
        let mut end_y = end.1;
        if start_y > end_y {
            start_y = end.1;
            end_y = start.1;
        }

        for y in start_y..=end_y {
            let entry = points_covered.entry((x, y)).or_insert(0);
            *entry += 1;
        }
    } else if start.1 == end.1 {
        let y = start.1;
        let mut start_x = start.0;
        let mut end_x = end.0;
        if start_x > end_x {
            start_x = end.0;
            end_x = start.0;
        }

        for x in start_x..=end_x {
            let entry = points_covered.entry((x, y)).or_insert(0);
            *entry += 1;
        }
    }
}

fn find_crossing_vents(points_covered: &HashMap<(u32, u32), u32>) -> u32 {
    points_covered
        .iter()
        .fold(0, |acc, (_, &count)| if count >= 2 { acc + 1 } else { acc })
}

fn part_one(input: &str) -> u32 {
    let mut points_covered: HashMap<(u32, u32), u32> = HashMap::new();

    for line in input.lines() {
        let mut points = line.split(" -> ");

        let mut start = points.next().unwrap().split(',');
        let start: (u32, u32) = (
            start.next().unwrap().parse().unwrap(),
            start.next().unwrap().parse().unwrap(),
        );

        let mut end = points.next().unwrap().split(',');
        let end: (u32, u32) = (
            end.next().unwrap().parse().unwrap(),
            end.next().unwrap().parse().unwrap(),
        );

        register_vent(&start, &end, &mut points_covered);
    }

    find_crossing_vents(&points_covered)
}

fn main() {
    let input = include_str!("input.txt");

    let count1 = part_one(input);

    println!(
        "The number of vertical and horizontal crossing vents is {}",
        count1
    );
}
