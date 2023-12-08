use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

type Node = char;

#[derive(Debug, PartialEq, Eq)]
struct Map {
    pattern: String,
    nodes: HashMap<Node, (Node, Node)>,
}

#[derive(Debug, PartialEq)]
struct NodeErr;

impl TryFrom<&str> for Map {
    type Error = NodeErr;

    // Required method
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut lines = value.lines();

        let pattern = lines.next().expect("must have pattern").to_owned();
        let _blank = lines.next().expect("Must have a least a blank line");
        // todo!();
        let nodes: HashMap<Node, (Node, Node)> = lines
            .map(|line| {
                //AAA = (BBB, BBB)
                //0123456789ABCDEF
                let chars = line.chars().collect::<Vec<_>>();

                let key: Node = chars[0usize];
                let l: Node = chars[7usize];
                let r: Node = chars[12usize];
                (key, (l, r))
            })
            .collect();

        Ok(Map { pattern, nodes })
    }
}
fn part1(input: &str) -> u32 {
    let map: Map = input.try_into().expect("Must be able to decode a map.");
    1u32
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn map_decode() {
        let input = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let actual = input.try_into();
        let expected = Ok(Map {
            pattern: String::from("LLR"),
            nodes: HashMap::from([('A', ('B', 'B')), ('B', ('A', 'Z')), ('Z', ('Z', 'Z'))]),
        });

        assert_eq!(actual, expected);
    }
    // #[test]
    // fn example() {
    //     let input = r"
    //     1abc2
    //     pqr3stu8vwx
    //     a1b2c3d4e5f
    //     treb7uchet";
    //     assert_eq!(part1(input), 142u32)
    // }
}
