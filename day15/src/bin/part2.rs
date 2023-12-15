use core::num::Wrapping;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Lens<'a> {
    label: &'a str,
    factor: u8,
}

#[derive(Clone, Debug, PartialEq)]
struct LensBox<'a> {
    lens: VecDeque<Lens<'a>>,
}

impl<'a> Default for LensBox<'a> {
    fn default() -> Self {
        Self {
            lens: VecDeque::new(),
        }
    }
}
#[derive(Debug, PartialEq)]
struct Row<'a> {
    boxes: HashMap<u8, LensBox<'a>>,
}

impl<'a> Default for Row<'a> {
    fn default() -> Self {
        let mut boxes = HashMap::new();
        for key in 0..=255 {
            boxes.insert(key, LensBox::default());
        }
        Row { boxes }
    }
}

impl<'a> Row<'a> {
    fn process(&mut self, instr: &'a Instruction) {
        let box_id = hash_day15(instr.label);
        // Drop if box not found
        dbg!(&instr);
        if let Some(b) = self.boxes.get_mut(&box_id) {
            match instr.operation {
                Operation::Equals(factor) => {
                    let new_lens = Lens {
                        label: instr.label,
                        factor,
                    };
                    dbg!(&new_lens);

                    if b.lens.contains(&new_lens) {
                        let index = b.lens.partition_point(|lens| lens.label == new_lens.label);
                        // push back .. dont distrub index
                        b.lens.push_back(new_lens);
                        b.lens.swap_remove_back(index);
                    } else {
                        b.lens.push_front(new_lens);
                    }
                }
                Operation::Subtract => {
                    todo!();
                }
            }
        }
    }
}

#[derive(Debug)]
enum Operation {
    Subtract,
    Equals(u8),
}

impl From<&str> for Operation {
    fn from(c: &str) -> Self {
        let mut chars = c.chars();
        match chars.next() {
            Some('-') => Operation::Subtract,
            Some('=') => {
                // Must be follow by at least one chars,
                // That char must decode to a digit.
                let factor = chars.next().unwrap().to_digit(10).unwrap() as u8;
                Operation::Equals(factor)
            }
            _ => {
                panic!("Could not decode instruction {c}");
            }
        }
    }
}

#[derive(Debug)]
struct Instruction<'a> {
    label: &'a str,
    operation: Operation,
}

impl<'a> From<&'a str> for Instruction<'a> {
    fn from(input: &'a str) -> Self {
        dbg!(input);

        let label = &input[0..=1];
        let mut chars = input.chars().skip(2).take(4);

        let operation = input[2..].into();

        Self { label, operation }
    }
}

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn part1(input: &str) -> u64 {
    let totals = input.lines().map(single_line).collect::<Vec<_>>();
    totals.iter().sum()
}

fn single_line(input: &str) -> u64 {
    input.split(',').map(|x| hash_day15(x) as u64).sum()
}

fn hash_day15(input: &str) -> u8 {
    let sum = input.bytes().fold(0, |cv, c| {
        let a = Wrapping(cv) + Wrapping(c);
        let b = a * Wrapping(17);
        b.0
    });

    sum
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn hash() {
        let input = r"HASH";
        assert_eq!(hash_day15(input), 52);
    }

    #[test]
    fn simple() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let total: u64 = input.split(',').map(|x| hash_day15(x) as u64).sum();

        assert_eq!(total, 1320);
    }

    #[test]
    fn mulit_line() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let total: u64 = part1(input);

        assert_eq!(total, 2 * 1320);
    }

    #[test]
    fn process() {
        // let process_strings: Vec<&str> = vec![
        //     // "rn=1", "cm-", "qp=3", "cm=2", "qp-", "pc=4", "ot=9", "ab=5", "pc-", "pc=6", "ot=7",
        //     "rn=1",
        // ];

        // let process_list = process_strings
        //     .iter()
        //     .map(|line| {
        //         let a = *line;
        //         a.into()
        //     })
        //     .collect::<Vec<_>>();

        // let mut iter = process_list.iter();
        let mut boxes = Row::default();

        // Step one
        // After "rn=1":
        let binding = Instruction::from("rn=1");
        boxes.process(&binding);

        let mut expected_boxes = Row::default();
        let expected_box0 = expected_boxes.boxes.get_mut(&0).unwrap();
        expected_box0.lens.push_front(Lens {
            label: "rn",
            factor: 1,
        });

        assert_eq!(boxes, expected_boxes);

        // Step two
        // After "cm-":
        let binding = Instruction::from("cm-");
        boxes.process(&binding);

        // No change
        assert_eq!(boxes, expected_boxes);
    }
}
