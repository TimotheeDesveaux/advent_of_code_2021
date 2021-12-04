struct Board {
    numbers: [i32; 5 * 5],
}

impl Board {
    fn from_input(input: &str) -> Board {
        let vec: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        Board {
            numbers: vec.try_into().unwrap(),
        }
    }

    fn draw_number(&mut self, number: &i32) -> bool {
        let index = self.numbers.iter().position(|n| n == number);
        match index {
            Some(i) => {
                self.numbers[i] = -1;
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
            let mut is_valid = true;
            for y in 0..5 {
                if self.get_number(x, y) != -1 {
                    is_valid = false;
                    break;
                }
            }

            if is_valid {
                return true;
            }
        }

        false
    }

    fn is_row_won(&self) -> bool {
        for y in 0..5 {
            let mut is_valid = true;
            for x in 0..5 {
                if self.get_number(x, y) != -1 {
                    is_valid = false;
                    break;
                }
            }

            if is_valid {
                return true;
            }
        }

        false
    }

    fn is_won(&self) -> bool {
        self.is_column_won() || self.is_row_won()
    }

    fn sum_unmarked(&self) -> i32 {
        self.numbers
            .iter()
            .fold(0, |acc, &n| if n != -1 { acc + n } else { acc })
    }
}

fn part_one(drawn_numbers: &Vec<i32>, boards: &mut Vec<Board>) {
    for number in drawn_numbers.iter() {
        for board in boards.iter_mut() {
            if board.draw_number(number) && board.is_won() {
                println!("The final score is {}", board.sum_unmarked() * number);
                return;
            }
        }
    }
}

fn main() {
    let input = include_str!("input.txt");

    let mut sections = input.split("\n\n");
    let drawn_numbers: Vec<i32> = sections
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    let mut boards: Vec<Board> = sections.map(|s| Board::from_input(s)).collect();

    part_one(&drawn_numbers, &mut boards);
}
