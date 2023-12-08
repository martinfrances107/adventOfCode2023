use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part2(input));
}

type Node = [char; 3];

#[derive(Debug, PartialEq, Eq)]
struct Map {
    pattern: String,
    nodes: HashMap<Node, (Node, Node)>,
}

impl Map {
    fn multiwalk(&self, starting_nodes: Vec<Node>) -> u128 {
        let mut nodes = starting_nodes;
        let mut count: u128 = 0;
        for direction in self.pattern.chars().cycle() {
            let new_nodes = nodes
                .iter()
                .map(|node| {
                    let Some((l, r)) = self.nodes.get(node) else {
                        dbg!(&node);
                        panic!("multiwalk: malformed node");
                    };
                    match direction {
                        'L' => *l,
                        'R' => *r,
                        _ => panic!("unexpected direction"),
                    }
                })
                .collect::<Vec<_>>();

            count += 1;

            if new_nodes.iter().all(|n| n[2] == 'Z') {
                break;
            }
            if count > 200_00_000_000 {
                panic!("opps looping");
            }
            nodes = new_nodes;
        }
        count
    }

    fn get_nodes_ending(&self, symbol: char) -> Vec<Node> {
        self.nodes
            .keys()
            .filter_map(|node| if node[2] == symbol { Some(*node) } else { None })
            .collect::<Vec<_>>()
    }
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
                //012____789__CDE
                let chars = line.chars().collect::<Vec<_>>();

                let key: Node = [chars[0], chars[1], chars[2]];
                let l: Node = [chars[7], chars[8], chars[9]];
                let r: Node = [chars[12], chars[13], chars[14]];
                (key, (l, r))
            })
            .collect();

        Ok(Map { pattern, nodes })
    }
}
fn part2(input: &str) -> u32 {
    let map: Map = input.try_into().expect("Must be able to decode a map.");

    let starting_nodes = map.get_nodes_ending('A');

    let n_steps = map.multiwalk(starting_nodes);

    n_steps as u32
}

#[cfg(test)]
mod test {

    const START: Node = ['A', 'A', 'A'];
    const END: Node = ['Z', 'Z', 'Z'];

    use super::*;

    #[test]
    fn map_decode() {
        let input = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let node_b = ['B', 'B', 'B'];
        let actual = input.try_into();
        let expected = Ok(Map {
            pattern: String::from("LLR"),
            nodes: HashMap::from([
                (START, (node_b, node_b)),
                (node_b, (START, END)),
                (END, (END, END)),
            ]),
        });

        assert_eq!(actual, expected);
    }

    //     Step 0: You are at 11A and 22A.
    // Step 1: You choose all of the left paths, leading you to 11B and 22B.
    // Step 2: You choose all of the right paths, leading you to 11Z and 22C.
    // Step 3: You choose all of the left paths, leading you to 11B and 22Z.
    // Step 4: You choose all of the right paths, leading you to 11Z and 22B.
    // Step 5: You choose all of the left paths, leading you to 11B and 22C.
    // Step 6: You choose all of the right paths, leading you to 11Z and 22Z.
    #[test]
    fn multi_walk() {
        let input = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let map: Map = input.try_into().expect("bad map");

        let expected_starting_nodes = vec![['1', '1', 'A'], ['2', '2', 'A']];
        let mut starting_nodes = map.get_nodes_ending('A');
        starting_nodes.sort();

        assert_eq!(starting_nodes, expected_starting_nodes);
        // let final_nodes = map.get_nodes_ending('Z');
        let expected = map.multiwalk(starting_nodes);
        assert_eq!(expected, 6);
    }
}
