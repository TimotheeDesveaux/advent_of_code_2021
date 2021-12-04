struct Board {
    numbers: [i32; 5 * 5],
    is_won: bool,
}

impl Board {
    fn from_input(input: &str) -> Board {
        let vec: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        Board {
            numbers: vec.try_into().unwrap(),
            is_won: false,
        }
    }

    fn draw_number(&mut self, number: &i32) -> bool {
        let index = self.numbers.iter().position(|n| n == number);
        match index {
            Some(i) => {
                self.numbers[i] = -1;
                self.is_won = self.is_row_won() || self.is_column_won();
                true
            }
            _ => false,
        }
    }

    fn get_number(&self, x: usize, y: usize) -> i32 {
        self.numbers[y * 5 + x]
    }

    fn is_column_won(&self) -> bool {
        for x in 0..5 {
            let mut is_complete = true;
            for y in 0..5 {
                if self.get_number(x, y) != -1 {
                    is_complete = false;
                    break;
                }
            }

            if is_complete {
                return true;
            }
        }

        false
    }

    fn is_row_won(&self) -> bool {
        for y in 0..5 {
            let mut is_complete = true;
            for x in 0..5 {
                if self.get_number(x, y) != -1 {
                    is_complete = false;
                    break;
                }
            }

            if is_complete {
                return true;
            }
        }

        false
    }

    fn sum_unmarked(&self) -> i32 {
        self.numbers
            .iter()
            .fold(0, |acc, &n| if n != -1 { acc + n } else { acc })
    }
}

fn part_one(input: &str) -> i32 {
    let mut sections = input.split("\n\n");
    let drawn_numbers: Vec<i32> = sections
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    let mut boards: Vec<Board> = sections.map(|s| Board::from_input(s)).collect();

    for number in drawn_numbers.iter() {
        for board in boards.iter_mut() {
            if board.draw_number(number) && board.is_won {
                return board.sum_unmarked() * number;
            }
        }
    }

    -1
}

fn part_two(input: &str) -> i32 {
    let mut sections = input.split("\n\n");
    let drawn_numbers: Vec<i32> = sections
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    let mut boards: Vec<Board> = sections.map(|s| Board::from_input(s)).collect();

    let mut unfinished_board = boards.len();
    for number in drawn_numbers.iter() {
        for board in boards.iter_mut().filter(|b| !b.is_won) {
            if board.draw_number(number) && board.is_won {
                unfinished_board -= 1;
                if unfinished_board == 0 {
                    return board.sum_unmarked() * number;
                }
            }
        }
    }

    -1
}

fn main() {
    let input = include_str!("input.txt");

    let score1 = part_one(input);
    let score2 = part_two(input);

    println!("The score of the winning board is {}", score1);
    println!("The score of the last remaining board is {}", score2);
}
