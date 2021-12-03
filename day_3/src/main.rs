fn weights_to_rates(weights: &Vec<u32>, threshold: &u32) -> (u32, u32) {
    let mut current_power = 0;
    let mut gamma_rate: u32 = 0;
    let mut epsilon_rate: u32 = 0;

    for weight in weights.iter().rev() {
        if weight > threshold {
            gamma_rate += 1 << current_power;
        } else {
            epsilon_rate += 1 << current_power;
        }

        current_power += 1;
    }

    (gamma_rate, epsilon_rate)
}

fn part_one(input: &str) -> u32 {
    let first = input.lines().next();
    let mut weights: Vec<u32> = vec![0; first.unwrap().len()];
    let mut number_of_line = 0;

    for line in input.lines() {
        number_of_line += 1;
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                weights[i] += 1;
            }
        }
    }

    let (gamma_rate, epsilon_rate) = weights_to_rates(&weights, &(number_of_line / 2));
    gamma_rate * epsilon_rate
}

fn main() {
    let input = include_str!("input.txt");

    let consumption1 = part_one(input);

    println!("{}", consumption1);
}
