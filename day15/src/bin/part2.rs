use core::num::Wrapping;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::os::unix::process;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Lens<'a> {
    label: &'a str,
    focal_length: u8,
}
impl<'a> Lens<'a> {
    fn label_matches(&self, label: &str) -> bool {
        self.label == label
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct LensBox<'a> {
    lens: VecDeque<Lens<'a>>,
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
    fn process(&mut self, input: &'a str) {
        let instr: Instruction = input.into();

        let box_id = hash_day15(instr.label);

        if let Some(b) = self.boxes.get_mut(&box_id) {
            match instr.operation {
                Operation::Equals(focal_length) => {
                    let mut found = false;
                    for lens in b.lens.iter_mut() {
                        if lens.label_matches(instr.label) {
                            // As per instuction update the focal length.
                            lens.focal_length = focal_length;
                            found = true;
                        }
                    }
                    // If new lens push to the back.
                    if !found {
                        b.lens.push_back(Lens {
                            label: instr.label,
                            focal_length,
                        })
                    }
                }
                Operation::Subtract => {
                    //get the position of any matching lens
                    b.lens.retain(|lens| !lens.label_matches(instr.label))
                }
            }
        }
    }

    fn focal_power(&self) -> u64 {
        println!("focal power");
        let mut total_power = 0u64;
        for box_index in 0..=255 {
            let box_number = (box_index as u64) + 1;
            let b = self.boxes.get(&box_index).unwrap();
            let focal_powers_for_box = b
                .lens
                .iter()
                .enumerate()
                .map(|(len_index, lens)| {
                    let lens_number = (len_index as u64) + 1;
                    println!("{}", &lens_number);
                    box_number * lens_number * (lens.focal_length as u64)
                })
                .collect::<Vec<u64>>();
            println!();
            total_power += focal_powers_for_box.iter().sum::<u64>();
        }
        total_power
    }
}

#[derive(Debug, Clone)]
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

#[derive(Clone, Debug)]
struct Instruction<'a> {
    label: &'a str,
    operation: Operation,
}

impl<'a> From<&'a str> for Instruction<'a> {
    fn from(input: &'a str) -> Self {
        match input.split_once('=') {
            Some((label, fl_str)) => {
                let fl = fl_str.chars().nth(0).unwrap().to_digit(10).unwrap();
                let instruction = Instruction {
                    label,
                    operation: Operation::Equals(fl as u8),
                };
                instruction
            }
            None => match input.split_once('-') {
                Some((label, _)) => Instruction {
                    operation: Operation::Subtract,
                    label,
                },
                None => {
                    panic!("could not decode");
                }
            },
        }
    }
}

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn part1(input: &str) -> u64 {
    // let process_strings: Vec<&'static str> = vec![
    //     "rn=1", "cm-", "qp=3", "cm=2", "qp-", "pc=4", "ot=9", "ab=5", "pc-", "pc=6", "ot=7",
    // ];

    let process_strings = input.split(',').collect::<Vec<_>>();
    dbg!(&process_strings);

    let mut row = Row::<'static>::default();
    for i in 0..process_strings.len() {
        row.process(&process_strings[i]);
    }

    row.focal_power()
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

    #[ignore]
    #[test]
    fn long_label() {
        let eq_str = "helloWord=66";
        let instruction: Instruction = eq_str.into();
        dbg!(instruction);
        let sub_str = "bababa-";
        let instruction: Instruction = sub_str.into();
        dbg!(instruction);
        assert!(false);
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
    fn process_steps() {
        let mut boxes = Row::default();

        // Step one
        // After "rn=1":
        boxes.process(&"rn=1");

        let mut expected_boxes = Row::default();
        let expected_box0 = expected_boxes.boxes.get_mut(&0).unwrap();
        expected_box0.lens.push_front(Lens {
            label: "rn",
            focal_length: 1,
        });

        assert_eq!(boxes, expected_boxes);

        // Step two
        // After "cm-":

        boxes.process(&"cm-");

        // No change
        assert_eq!(boxes, expected_boxes);

        // Step three
        // After "qp=3":
        boxes.process("qp=3");

        let expected_box1 = expected_boxes.boxes.get_mut(&1).unwrap();

        expected_box1.lens.push_front(Lens {
            label: "qp",
            focal_length: 3, // TODO 3 is not checked here.
        });
        assert_eq!(boxes, expected_boxes);
    }

    #[test]
    fn process() {
        let process_strings: Vec<&'static str> = vec![
            "rn=1", "cm-", "qp=3", "cm=2", "qp-", "pc=4", "ot=9", "ab=5", "pc-", "pc=6", "ot=7",
        ];

        let mut row = Row::<'static>::default();
        for i in 0..process_strings.len() {
            row.process(&process_strings[i]);
        }

        // These boxes match the test results.
        // dbg!(row.boxes.get(&0));
        // dbg!(row.boxes.get(&1));
        // dbg!(row.boxes.get(&2));
        // dbg!(row.boxes.get(&3));

        assert_eq!(row.focal_power(), 146);
    }
}
