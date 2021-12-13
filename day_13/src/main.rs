fn apply_fold(dots: &mut Vec<(u32, u32)>, orientation: char, origin: u32) {
    for dot in dots.iter_mut() {
        if orientation == 'x' && dot.0 > origin {
            *dot = (origin - (dot.0 - origin), dot.1);
        } else if orientation == 'y' && dot.1 > origin {
            *dot = (dot.0, origin - (dot.1 - origin));
        }
    }
}

fn string_code(dots: &Vec<(u32, u32)>) -> String {
    let mut width = dots[0].0;
    let mut height = dots[0].1;

    for dot in dots.iter().skip(1) {
        width = u32::max(width, dot.0);
        height = u32::max(height, dot.1);
    }

    let width = (width + 2) as usize;
    let height = (height + 1) as usize;

    let mut code = vec!['.'; width * height - 1];

    for i in 1..height {
        code[i * width - 1] = '\n';
    }

    for dot in dots {
        let y = dot.1 as usize;
        let x = dot.0 as usize;
        code[y * width + x] = '#';
    }

    code.iter().collect::<String>()
}

fn part_one(input: &str) -> usize {
    let (dots_input, folds_input) = input.split_once("\n\n").unwrap();

    let mut dots: Vec<(u32, u32)> = Vec::new();
    for dot in dots_input.lines() {
        let dot = dot.split_once(',').unwrap();
        let dot: (u32, u32) = (dot.0.parse().unwrap(), dot.1.parse().unwrap());
        dots.push(dot);
    }

    let fold = folds_input.lines().next().unwrap();
    let (orientation, origin) = fold.split_once('=').unwrap();
    let orientation = orientation.chars().last().unwrap();
    let origin: u32 = origin.parse().unwrap();

    apply_fold(&mut dots, orientation, origin);

    dots.sort();
    dots.dedup();

    dots.len()
}

fn part_two(input: &str) -> String {
    let (dots_input, folds_input) = input.split_once("\n\n").unwrap();

    let mut dots: Vec<(u32, u32)> = Vec::new();
    for dot in dots_input.lines() {
        let dot = dot.split_once(',').unwrap();
        let dot: (u32, u32) = (dot.0.parse().unwrap(), dot.1.parse().unwrap());
        dots.push(dot);
    }

    for fold in folds_input.lines() {
        let (orientation, origin) = fold.split_once('=').unwrap();
        let orientation = orientation.chars().last().unwrap();
        let origin: u32 = origin.parse().unwrap();

        apply_fold(&mut dots, orientation, origin);
    }

    dots.sort();
    dots.dedup();

    string_code(&dots)
}

fn main() {
    let input = include_str!("input.txt");

    let number_of_dots = part_one(input);
    let code = part_two(input);

    println!(
        "The number of points visible after the first fold is {}",
        number_of_dots
    );
    println!("The code is:\n{}", code);
}
