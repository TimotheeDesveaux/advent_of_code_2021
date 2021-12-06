use std::collections::HashMap;

fn register_vent(
    start: &(u32, u32),
    end: &(u32, u32),
    points_covered: &mut HashMap<(u32, u32), u32>,
) {
    let mut cur = *start;

    while cur != *end {
        let entry = points_covered.entry(cur).or_insert(0);
        *entry += 1;

        if cur.0 < end.0 {
            cur.0 += 1;
        } else if cur.0 > end.0 {
            cur.0 -= 1;
        }

        if cur.1 < end.1 {
            cur.1 += 1;
        } else if cur.1 > end.1 {
            cur.1 -= 1;
        }
    }

    let entry = points_covered.entry(cur).or_insert(0);
    *entry += 1;
}

fn find_crossing_vents(input: &str, consider_diagonals: bool) -> u32 {
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

        if consider_diagonals || start.0 == end.0 || start.1 == end.1 {
            register_vent(&start, &end, &mut points_covered);
        }
    }

    points_covered
        .iter()
        .fold(0, |acc, (_, &count)| if count >= 2 { acc + 1 } else { acc })
}

fn part_one(input: &str) -> u32 {
    find_crossing_vents(input, false)
}

fn part_two(input: &str) -> u32 {
    find_crossing_vents(input, true)
}

fn main() {
    let input = include_str!("input.txt");

    let count1 = part_one(input);
    let count2 = part_two(input);

    println!(
        "The number of vertical and horizontal crossing vents is {}",
        count1
    );
    println!("The number crossing vents is {}", count2);
}
