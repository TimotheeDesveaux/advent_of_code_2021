fn count_increase_windowed(depths: &Vec<i32>, window_size: usize) -> u32 {
    if depths.len() <= window_size {
        return 0;
    }

    let mut count: u32 = 0;

    for i in (window_size)..depths.len() {
        if &depths[(i - window_size)..(i)].iter().sum::<i32>()
            < &depths[(i - window_size + 1)..(i + 1)].iter().sum::<i32>()
        {
            count += 1;
        }
    }

    count
}

fn part_one(depths: &Vec<i32>) -> u32 {
    count_increase_windowed(&depths, 1)
}

fn part_two(depths: &Vec<i32>) -> u32 {
    count_increase_windowed(&depths, 3)
}

fn main() {
    let input = include_str!("input.txt");

    let depths: Vec<i32> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();

    let count1 = part_one(&depths);
    let count2 = part_two(&depths);

    println!(
        "There are {} measurements larger than the previous one.",
        count1
    );

    println!("There are {} sums larger than the previous one.", count2);
}
