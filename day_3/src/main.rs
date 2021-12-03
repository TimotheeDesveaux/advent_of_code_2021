fn weights_to_rates(weights: &Vec<u32>, threshold: usize) -> (u32, u32) {
    let mut current_power = 0;
    let mut gamma_rate: u32 = 0;
    let mut epsilon_rate: u32 = 0;
    let threshold: u32 = threshold.try_into().unwrap();

    for weight in weights.iter().rev() {
        if weight > &threshold {
            gamma_rate += 1 << current_power;
        } else {
            epsilon_rate += 1 << current_power;
        }

        current_power += 1;
    }

    (gamma_rate, epsilon_rate)
}

fn bit_array_to_decimal(bits: &str) -> u32 {
    let mut current_power = -1;
    bits.chars().rev().fold(0, |acc, bit| {
        current_power += 1;
        if bit == '1' {
            acc + (1 << current_power)
        } else {
            acc
        }
    })
}

fn numbers_to_weights(numbers: &Vec<&str>) -> Vec<u32> {
    let mut weights: Vec<u32> = vec![0; numbers[0].len()];

    for number in numbers {
        for (i, c) in number.chars().enumerate() {
            if c == '1' {
                weights[i] += 1;
            }
        }
    }

    weights
}

fn part_one(input: &str) -> u32 {
    let weights = numbers_to_weights(&input.lines().collect());

    let (gamma_rate, epsilon_rate) = weights_to_rates(&weights, input.lines().count() / 2);
    gamma_rate * epsilon_rate
}

fn part_two(input: &str) -> u32 {
    let mut filtered_lines_o2: Vec<&str> = input.lines().collect();
    let mut filtered_lines_co2: Vec<&str> = input.lines().collect();

    for i in 0..filtered_lines_o2[0].len() {
        let o2_weights = numbers_to_weights(&filtered_lines_o2);
        let o2_threshold: u32 = ((filtered_lines_o2.len() + 1) / 2).try_into().unwrap();
        if filtered_lines_o2.len() > 1 {
            filtered_lines_o2 = filtered_lines_o2
                .into_iter()
                .filter(|l| {
                    let c = l.chars().nth(i).unwrap();
                    if o2_weights[i] < o2_threshold {
                        c == '0'
                    } else {
                        c == '1'
                    }
                })
                .collect();
        }

        let co2_weights = numbers_to_weights(&filtered_lines_co2);
        let co2_threshold: u32 = ((filtered_lines_co2.len() + 1) / 2).try_into().unwrap();
        if filtered_lines_co2.len() > 1 {
            filtered_lines_co2 = filtered_lines_co2
                .into_iter()
                .filter(|l| {
                    let c = l.chars().nth(i).unwrap();
                    if co2_weights[i] < co2_threshold {
                        c == '1'
                    } else {
                        c == '0'
                    }
                })
                .collect();
        }
    }

    bit_array_to_decimal(filtered_lines_o2[0]) * bit_array_to_decimal(filtered_lines_co2[0])
}

fn main() {
    let input = include_str!("input.txt");

    let consumption = part_one(input);
    let life_support = part_two(input);

    println!("The power consumption of the submarine is {}", consumption);
    println!(
        "The life suport rating of the submarine is {}",
        life_support
    );
}
