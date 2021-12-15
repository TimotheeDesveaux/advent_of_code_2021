use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct State {
    cost: u32,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part_one(input: &str) -> u32 {
    let width = input.lines().count();
    let start = (0, 0);
    let goal = (width - 1, width - 1);

    let risk_levels: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut dist: Vec<Vec<u32>> = vec![vec![u32::MAX; width]; width];
    dist[0][0] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return cost;
        } else if cost > dist[position.1][position.0] {
            continue;
        }

        let x: i32 = position.0.try_into().unwrap();
        let y: i32 = position.1.try_into().unwrap();
        for (adj_x, adj_y) in [(x, y - 1), (x + 1, y), (x, y + 1), (x, y - 1)] {
            if adj_x >= 0
                && adj_x < width.try_into().unwrap()
                && adj_y >= 0
                && adj_y < width.try_into().unwrap()
            {
                let adj_x: usize = adj_x.try_into().unwrap();
                let adj_y: usize = adj_y.try_into().unwrap();
                let next = State {
                    cost: cost + risk_levels[adj_y][adj_x],
                    position: (adj_x, adj_y),
                };
                if next.cost < dist[adj_y][adj_x] {
                    dist[adj_y][adj_x] = next.cost;
                    heap.push(next);
                }
            }
        }
    }

    unreachable!();
}

fn main() {
    let input = include_str!("example.txt");

    let test = part_one(input);

    println!("{}", test);
}
