use core::fmt::Display;
use core::fmt::Formatter;

fn main() {
    let input = include_str!("./sample.txt");
    println!("{:?}", part1(input));
}

#[derive(Debug)]
struct Dir {
    row_inc: i64,
    col_inc: i64,
}
// clock rotation: N, E, S, W
// (row increment, col increment)
const DIRECTION: [Dir; 4] = [
    Dir {
        row_inc: -1,
        col_inc: 0,
    },
    Dir {
        row_inc: 0,
        col_inc: 1,
    },
    Dir {
        row_inc: 1,
        col_inc: 0,
    },
    Dir {
        row_inc: 0,
        col_inc: -1,
    },
];

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

#[derive(Debug)]
struct PipeMap {
    row: Vec<Vec<char>>,
    start_row_index: usize,
    start_col_index: usize,
    output: Vec<Vec<char>>,
    next_digit: usize,
}

impl Display for PipeMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for (i, r) in self.output.iter().enumerate() {
            write!(f, "{i} :");
            for c in r.iter() {
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
#[derive(Debug, Eq, PartialEq)]
struct State {
    row_index: usize,
    col_index: usize,
    blocked_direction: Option<usize>,
    distance: u32,
}

impl State {
    fn position_match(&self, other: &Self) -> bool {
        self.row_index == other.row_index && self.col_index == other.col_index
    }
}

impl From<&str> for PipeMap {
    fn from(input: &str) -> Self {
        let mut start_row_index = None;
        let mut start_col_index = None;
        let mut output = vec![];
        let row = input
            .lines()
            .enumerate()
            .map(|(row_index, line)| {
                output.push(vec!['.'; line.len()]);
                line.chars()
                    .enumerate()
                    .map(|(col_index, c)| {
                        if c == 'S' {
                            dbg!(row_index);
                            dbg!(col_index);
                            start_row_index = Some(row_index);
                            start_col_index = Some(col_index);
                        }
                        c
                    })
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>();

        match (start_row_index, start_col_index) {
            (Some(start_row_index), Some(start_col_index)) => {
                output[start_row_index][start_col_index] = 'S';

                Self {
                    row,
                    start_row_index,
                    start_col_index,
                    output,
                    next_digit: 0,
                }
            }
            _ => {
                panic!("Must have found a start symbol");
            }
        }
    }
}

impl PipeMap {
    fn at_start(&self, row_index: usize, col_index: usize) -> bool {
        self.start_row_index == row_index && self.start_col_index == col_index
    }
    fn walk(&mut self, state: State) -> State {
        // walk round the clock looking for the exit.
        let mut next_blocked_direction = state.blocked_direction;
        let mut row_index = 0usize;
        let mut col_index = 0usize;
        for (i, d) in DIRECTION.iter().enumerate() {
            if Some(i) != state.blocked_direction {
                row_index = (state.row_index as i64 + d.row_inc) as usize;
                if let Some(row) = self.row.get(row_index) {
                    col_index = (state.col_index as i64 + d.col_inc) as usize;

                    // dbg!(&row_index);
                    // dbg!(&col_index);
                    // if row_index > 122 {}
                    // pe - The current pipe element under consideration.
                    if let Some(pe) = row.get(col_index) {
                        // if we have found the exit and can compute the next
                        // backward direction
                        match i {
                            0 => {
                                if *pe == '|' || *pe == 'F' || *pe == '7' {
                                    dbg!("north bound");
                                    next_blocked_direction = Some(2);

                                    self.output[row_index][col_index] = *pe;

                                    break;
                                }
                            }
                            1 => {
                                if *pe == '-' || *pe == '7' || *pe == 'J' {
                                    dbg!("east bound");
                                    next_blocked_direction = Some(3);
                                    self.output[row_index][col_index] = *pe;

                                    break;
                                }
                            }
                            2 => {
                                if *pe == '|' || *pe == 'J' || *pe == 'L' {
                                    dbg!("sound bound");
                                    next_blocked_direction = Some(0);
                                    self.output[row_index][col_index] = *pe;

                                    break;
                                }
                            }
                            3 => {
                                if *pe == '-' || *pe == 'F' || *pe == 'L' {
                                    dbg!("west bound");
                                    next_blocked_direction = Some(1);
                                    self.output[row_index][col_index] = *pe;

                                    break;
                                }
                            }
                            _ => {
                                panic!("invalid direction");
                            }
                        };
                    }
                }
            }
        }

        // For debug fill the distance into the map
        // self.row[y as usize][x as usize] = '*';
        State {
            distance: state.distance + 1,
            row_index,
            col_index,
            blocked_direction: next_blocked_direction,
        }
    }
}

fn part1(input: &str) -> u32 {
    let mut map: PipeMap = input.into();
    let mut first_walker = State {
        distance: 0,
        row_index: map.start_row_index,
        col_index: map.start_col_index,
        blocked_direction: None,
    };

    // Advance first machine into the pipe.
    first_walker = map.walk(first_walker);

    // which path should be block when starting the second walker?
    dbg!(first_walker.blocked_direction);
    let bd = match first_walker.blocked_direction {
        Some(0) => Some(2),
        Some(1) => Some(3),
        Some(2) => Some(0),
        Some(3) => Some(1),
        _ => panic!("invalid block direction to turn around."),
    };

    let mut second_walker = State {
        distance: 0,
        row_index: map.start_row_index,
        col_index: map.start_col_index,
        blocked_direction: bd,
    };
    //To keep in sync also push the second walker down the pipe.
    second_walker = map.walk(second_walker);

    let mut lc = 0;
    loop {
        // Advance each walker until the share the same location again .. at the mid point of the loop.
        first_walker = map.walk(first_walker);
        // second_walker = map.walk(second_walker);

        if first_walker.position_match(&second_walker) {
            break;
        }

        lc += 1;
        if lc > 60 {
            break;
        }
    }
    // dbg!(&first_walker);
    // assert_eq!(first_walker.distance, second_walker.distance);
    println!("{map}");
    first_walker.distance
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn single_step() {
        let lines = r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let mut map: PipeMap = lines.into();

        let start = State {
            distance: 0,
            row_index: map.start_row_index,
            col_index: map.start_col_index,
            blocked_direction: None,
        };

        let first_path_state = map.walk(start);
        dbg!(&first_path_state);

        assert_eq!(
            first_path_state,
            State {
                distance: 1,
                row_index: 2,
                col_index: 1,
                blocked_direction: Some(3)
            }
        );

        let second_start = State {
            distance: 0,
            row_index: map.start_row_index,
            col_index: map.start_col_index,
            blocked_direction: Some(1),
        };
        let second_path_state = map.walk(second_start);
        assert_eq!(
            second_path_state,
            State {
                distance: 1,
                row_index: 3,
                col_index: 0,
                blocked_direction: Some(0)
            }
        );
    }

    #[test]
    fn single_step_edge_case() {
        let lines = r"..F7.
.FJ|.
FJ.L7
|F--S
LJ...";

        let mut map: PipeMap = lines.into();

        assert_eq!(map.start_row_index, 3);
        assert_eq!(map.start_col_index, 4);
        let start = State {
            distance: 0,
            row_index: map.start_row_index,
            col_index: map.start_col_index,
            blocked_direction: None,
        };

        let first_path_state = map.walk(start);
        dbg!(&first_path_state);

        assert_eq!(
            first_path_state,
            State {
                distance: 1,
                row_index: 2,
                col_index: 4,
                blocked_direction: Some(2)
            }
        );

        let second_start = State {
            distance: 0,
            row_index: map.start_row_index,
            col_index: map.start_col_index,
            blocked_direction: Some(0),
        };
        let second_path_state = map.walk(second_start);
        assert_eq!(
            second_path_state,
            State {
                distance: 1,
                row_index: 3,
                col_index: 3,
                blocked_direction: Some(1)
            }
        );

        let dist1 = part1(lines);
        let dist2 = part1(lines);

        assert_eq!(dist1, 8);
        assert_eq!(dist2, 8);
    }

    #[test]
    fn example1() {
        let lines = r".....
.S-7.
.|.|.
.L-J.
.....";

        assert_eq!(part1(lines), 4);
    }

    // Same patter as example1 but which clutter.
    fn obsquered_loop() {
        let lines = r"-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

        assert_eq!(part1(lines), 4);
    }

    #[test]
    fn two_loops_unknown_length() {
        let loop1 = r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        let loop2 = r"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

        let dist1 = part1(loop1);
        let dist2 = part1(loop2);

        assert_eq!(dist1, dist2);
    }
    #[test]
    fn example2() {
        let lines = r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        assert_eq!(part1(lines), 8);
    }
}
