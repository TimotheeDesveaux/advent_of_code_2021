use std::num::ParseIntError;
use std::str::FromStr;

const WIDTH: usize = 10;

struct Octopuses {
    energy_levels: [u8; WIDTH * WIDTH],
}

impl FromStr for Octopuses {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec: Vec<u8> = s
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
            .collect();

        Ok(Octopuses {
            energy_levels: vec.try_into().unwrap(),
        })
    }
}

impl Octopuses {
    fn get_energy_level(&self, x: usize, y: usize) -> u8 {
        self.energy_levels[y * WIDTH + x]
    }

    fn set_energy_level(&mut self, x: usize, y: usize, energy_level: u8) {
        self.energy_levels[y * WIDTH + x] = energy_level
    }

    fn increase_energy_level(&mut self, x: usize, y: usize) -> u8 {
        let energy_level = (self.get_energy_level(x, y) + 1) % 10;
        self.set_energy_level(x, y, energy_level);

        energy_level
    }

    fn increase_adjacent_energy_level(
        &mut self,
        flash_positions: &mut Vec<(usize, usize)>,
        x: usize,
        y: usize,
    ) {
        let x: i8 = x.try_into().unwrap();
        let y: i8 = y.try_into().unwrap();
        let directions = [
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ];

        for direction in directions {
            let adj_x = x + direction.0;
            let adj_y = y + direction.1;
            if adj_x >= 0
                && adj_x < WIDTH.try_into().unwrap()
                && adj_y >= 0
                && adj_y < WIDTH.try_into().unwrap()
            {
                let adj_x: usize = adj_x.try_into().unwrap();
                let adj_y: usize = adj_y.try_into().unwrap();
                if self.get_energy_level(adj_x, adj_y) > 0 {
                    if self.increase_energy_level(adj_x, adj_y) == 0 {
                        flash_positions.push((adj_x, adj_y));
                    }
                }
            }
        }
    }

    fn next_step(&mut self) -> u32 {
        let mut flash_positions: Vec<(usize, usize)> = Vec::new();

        for y in 0..WIDTH {
            for x in 0..WIDTH {
                if self.increase_energy_level(x, y) == 0 {
                    flash_positions.push((x, y));
                }
            }
        }

        let mut number_of_flash = 0;
        while !flash_positions.is_empty() {
            let pos = flash_positions.pop().unwrap();
            self.increase_adjacent_energy_level(&mut flash_positions, pos.0, pos.1);
            number_of_flash += 1;
        }

        number_of_flash
    }
}

fn part_one(input: &str) -> u32 {
    let mut octopuses: Octopuses = input.parse().unwrap();
    let mut total_number_of_flash = 0;

    for _ in 0..100 {
        total_number_of_flash += octopuses.next_step();
    }

    total_number_of_flash
}

fn part_two(input: &str) -> u32 {
    let mut octopuses: Octopuses = input.parse().unwrap();
    let mut step = 1;

    while octopuses.next_step() != (WIDTH * WIDTH).try_into().unwrap() {
        step += 1;
    }

    step
}

fn main() {
    let input = include_str!("input.txt");

    let number_of_flash = part_one(input);
    let sync_step = part_two(input);

    println!("The number of flash after 100 step is {}", number_of_flash);
    println!(
        "The first step during which all octopuses flash is {}",
        sync_step
    );
}
