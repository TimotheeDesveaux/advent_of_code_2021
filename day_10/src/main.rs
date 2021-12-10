fn part_one(input: &str) -> u32 {
    let mut score = 0;

    for line in input.lines() {
        let mut chunks_stack: Vec<char> = Vec::new();
        for c in line.chars() {
            match c {
                '(' => chunks_stack.push(')'),
                '[' => chunks_stack.push(']'),
                '{' => chunks_stack.push('}'),
                '<' => chunks_stack.push('>'),
                c => match chunks_stack.pop() {
                    Some(closing) if closing == c => continue,
                    Some(_) => match c {
                        ')' => {
                            score += 3;
                            break;
                        }
                        ']' => {
                            score += 57;
                            break;
                        }
                        '}' => {
                            score += 1197;
                            break;
                        }
                        '>' => {
                            score += 25137;
                            break;
                        }
                        _ => panic!(),
                    },
                    _ => panic!(),
                },
            }
        }
    }

    score
}

fn part_two(input: &str) -> u64 {
    let mut scores: Vec<u64> = Vec::new();

    for line in input.lines() {
        let mut chunks_stack: Vec<char> = Vec::new();
        let mut error = false;

        for c in line.chars() {
            match c {
                '(' => chunks_stack.push(')'),
                '[' => chunks_stack.push(']'),
                '{' => chunks_stack.push('}'),
                '<' => chunks_stack.push('>'),
                c => match chunks_stack.pop() {
                    Some(closing) if closing == c => continue,
                    _ => {
                        error = true;
                        break;
                    }
                },
            }
        }

        if !error {
            scores.push(chunks_stack.iter().rev().fold(0, |acc, c| {
                acc * 5
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => panic!(),
                    }
            }))
        }
    }

    scores.sort();
    scores[scores.len() / 2]
}

fn main() {
    let input = include_str!("input.txt");

    let error_score = part_one(input);
    let middle_score = part_two(input);

    println!("The total syntax error score is {}", error_score);
    println!("The middle of autocomplete scores is {}", middle_score);
}
