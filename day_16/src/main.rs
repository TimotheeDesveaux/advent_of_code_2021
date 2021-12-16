#[derive(Debug)]
struct Packet {
    version: u8,
    content: Content,
}

impl Packet {
    fn parse(bits: &[u8], ptr: &mut usize) -> Packet {
        let version: u8 = bits_to_decimal(take_and_advance(bits, ptr, 3))
            .try_into()
            .unwrap();
        let content = Content::parse(bits, ptr);

        Packet { version, content }
    }

    fn sum_versions(&self) -> u32 {
        match &self.content {
            Content::Literal(_) => self.version as u32,
            Content::Operator(operator) => operator
                .sub_packets
                .iter()
                .fold(self.version as u32, |acc, p| acc + p.sum_versions()),
        }
    }

    fn eval(&self) -> u64 {
        self.content.eval()
    }
}

#[derive(Debug)]
enum Content {
    Literal(u64),
    Operator(Operator),
}

impl Content {
    fn parse(bits: &[u8], ptr: &mut usize) -> Content {
        let type_id: u8 = bits_to_decimal(take_and_advance(bits, ptr, 3))
            .try_into()
            .unwrap();

        if type_id == 4 {
            let mut literal: u64 = 0;
            loop {
                let last = take_one_and_advance(bits, ptr);
                literal = (literal << 4) + bits_to_decimal(take_and_advance(bits, ptr, 4));
                if last == 0 {
                    break Self::Literal(literal);
                }
            }
        } else {
            Self::Operator(Operator::parse(bits, ptr, type_id))
        }
    }

    fn eval(&self) -> u64 {
        match self {
            Self::Literal(n) => *n,
            Self::Operator(operator) => operator.eval(),
        }
    }
}

#[derive(Debug)]
struct Operator {
    operator_type: OperatorType,
    sub_packets: Vec<Packet>,
}

impl Operator {
    fn parse(bits: &[u8], ptr: &mut usize, type_id: u8) -> Operator {
        let mut sub_packets: Vec<Packet> = Vec::new();
        let length_type_id = take_one_and_advance(bits, ptr);

        if length_type_id == 0 {
            let subpackets_length: usize = bits_to_decimal(take_and_advance(bits, ptr, 15))
                .try_into()
                .unwrap();
            let start = *ptr;

            while *ptr - start < subpackets_length {
                sub_packets.push(Packet::parse(bits, ptr));
            }
        } else {
            let subpackets_number = bits_to_decimal(take_and_advance(bits, ptr, 11));
            for _ in 0..subpackets_number {
                sub_packets.push(Packet::parse(bits, ptr));
            }
        }

        let operator_type = match type_id {
            0 => OperatorType::Sum,
            1 => OperatorType::Product,
            2 => OperatorType::Mininumum,
            3 => OperatorType::Maximum,
            5 => OperatorType::Greater,
            6 => OperatorType::Lower,
            7 => OperatorType::Equal,
            _ => panic!(),
        };

        Operator {
            operator_type,
            sub_packets,
        }
    }

    fn eval(&self) -> u64 {
        match self.operator_type {
            OperatorType::Sum => self.sub_packets.iter().fold(0, |acc, p| acc + p.eval()),
            OperatorType::Product => self.sub_packets.iter().fold(1, |acc, p| acc * p.eval()),
            OperatorType::Mininumum => self.sub_packets.iter().map(|p| p.eval()).min().unwrap(),
            OperatorType::Maximum => self.sub_packets.iter().map(|p| p.eval()).max().unwrap(),
            OperatorType::Greater => {
                assert!(self.sub_packets.len() == 2);
                if self.sub_packets[0].eval() > self.sub_packets[1].eval() {
                    1
                } else {
                    0
                }
            }
            OperatorType::Lower => {
                assert!(self.sub_packets.len() == 2);
                if self.sub_packets[0].eval() < self.sub_packets[1].eval() {
                    1
                } else {
                    0
                }
            }
            OperatorType::Equal => {
                assert!(self.sub_packets.len() == 2);
                if self.sub_packets[0].eval() == self.sub_packets[1].eval() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

#[derive(Debug)]
enum OperatorType {
    Sum,
    Product,
    Mininumum,
    Maximum,
    Greater,
    Lower,
    Equal,
}

fn take_and_advance<'a>(bits: &'a [u8], ptr: &mut usize, length: usize) -> &'a [u8] {
    let res = &bits[*ptr..(*ptr + length)];
    *ptr += length;
    res
}

fn take_one_and_advance(bits: &[u8], ptr: &mut usize) -> u8 {
    take_and_advance(bits, ptr, 1)[0]
}

fn hex_to_bits(s: &str) -> Vec<u8> {
    s.chars()
        .flat_map(|c| match c {
            '0' => [0, 0, 0, 0],
            '1' => [0, 0, 0, 1],
            '2' => [0, 0, 1, 0],
            '3' => [0, 0, 1, 1],
            '4' => [0, 1, 0, 0],
            '5' => [0, 1, 0, 1],
            '6' => [0, 1, 1, 0],
            '7' => [0, 1, 1, 1],
            '8' => [1, 0, 0, 0],
            '9' => [1, 0, 0, 1],
            'A' => [1, 0, 1, 0],
            'B' => [1, 0, 1, 1],
            'C' => [1, 1, 0, 0],
            'D' => [1, 1, 0, 1],
            'E' => [1, 1, 1, 0],
            'F' => [1, 1, 1, 1],
            _ => panic!(),
        })
        .collect()
}

fn bits_to_decimal(bits: &[u8]) -> u64 {
    bits.iter().fold(0, |acc, &b| (acc << 1) + b as u64)
}

fn part_one(input: &str) -> u32 {
    let bits = hex_to_bits(input.trim());
    let packet = Packet::parse(&bits, &mut 0);

    packet.sum_versions()
}

fn part_two(input: &str) -> u64 {
    let bits = hex_to_bits(input.trim());
    let packet = Packet::parse(&bits, &mut 0);

    packet.eval()
}

fn main() {
    let input = include_str!("input.txt");

    let versions_sum = part_one(input);
    let eval = part_two(input);

    println!("The sum of the version of all packets is {}", versions_sum);
    println!("The evaluation of the transmission is {}", eval);
}
