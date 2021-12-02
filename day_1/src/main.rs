use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut previous_depth = -1;
    let mut count_increase = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let depth: i32 = ip.trim().parse().expect("Line should be a number");
                if previous_depth != -1 && previous_depth < depth {
                    count_increase += 1;
                }
                previous_depth = depth;
            }
        }
    }

    println!(
        "There are {} measurements larger than the previous one.",
        count_increase
    );
}
