use core::fmt::Display;
use core::fmt::Formatter;

fn main() {
    println!("\x1b[0;31mSO\x1b[0m");
    let input = include_str!("./input1.txt");
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

#[derive(Debug)]
struct PipeMap {
    input: Vec<Vec<char>>,
    output: Vec<Vec<char>>,
    start_row_index: usize,
    start_col_index: usize,
}

impl Display for PipeMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for r in self.output.iter() {
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
    // The pipe element at the center of the kernel.
    pe: char,
    row_index: usize,
    col_index: usize,
    in_dir: Option<usize>,
    distance: u32,
}

impl From<&str> for PipeMap {
    fn from(input: &str) -> Self {
        let mut start_row_index = None;
        let mut start_col_index = None;
        let mut output = vec![];
        let input = input
            .lines()
            .enumerate()
            .map(|(row_index, line)| {
                output.push(vec!['.'; line.len()]);
                line.chars()
                    .enumerate()
                    .map(|(col_index, c)| {
                        if c == 'S' {
                            start_row_index = Some(row_index);
                            start_col_index = Some(col_index);
                        }
                        c
                    })
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>();
        println!("row, col {:#?} {:#?}", start_row_index, start_col_index);
        match (start_row_index, start_col_index) {
            (Some(start_row_index), Some(start_col_index)) => {
                output[start_row_index][start_col_index] = 'S';

                Self {
                    input,
                    start_row_index,
                    start_col_index,
                    output,
                    // next_digit: 0,
                }
            }
            _ => {
                panic!("Must have found a start symbol");
            }
        }
    }
}

fn distance_to_char(distance: u32) -> char {
    format!("{:01x}", (distance % 16))
        .chars()
        .nth(0)
        .expect("error compting distance as a char")
}
impl PipeMap {
    fn at_start(&self, row_index: usize, col_index: usize) -> bool {
        self.start_row_index == row_index && self.start_col_index == col_index
    }

    // walk round the clock looking for the exit.
    fn walk(&mut self, state: State) -> State {
        for (dir_test, d) in DIRECTION.iter().enumerate() {
            if Some(dir_test) != state.in_dir {
                if let Some(search_row_index) = (state.row_index as i64).checked_add(d.row_inc) {
                    let search_row_index = search_row_index as usize;
                    if let Some(row) = self.input.get(search_row_index) {
                        if let Some(search_col_index) =
                            (state.col_index as i64).checked_add(d.col_inc)
                        {
                            let search_col_index = search_col_index as usize;
                            if let Some(search_pe) = row.get(search_col_index) {
                                match (dir_test, state.pe) {
                                    (0, 'S') | (0, 'J') | (0, '|') | (0, 'L') => {
                                        if *search_pe == '7'
                                            || *search_pe == '|'
                                            || *search_pe == 'F'
                                            || *search_pe == 'S'
                                        {
                                            self.output[state.row_index][state.col_index] =
                                                state.pe;
                                            return State {
                                                pe: *search_pe,
                                                distance: state.distance + 1,
                                                row_index: search_row_index,
                                                col_index: search_col_index,
                                                in_dir: Some(2),
                                            };
                                        }
                                    }
                                    (1, 'S') | (1, 'L') | (1, '-') | (1, 'F') => {
                                        if *search_pe == 'J'
                                            || *search_pe == '-'
                                            || *search_pe == '7'
                                            || *search_pe == 'S'
                                        {
                                            self.output[state.row_index][state.col_index] =
                                                state.pe;
                                            return State {
                                                pe: *search_pe,
                                                distance: state.distance + 1,
                                                row_index: search_row_index,
                                                col_index: search_col_index,
                                                in_dir: Some(3),
                                            };
                                        }
                                    }
                                    (2, 'S') | (2, '7') | (2, '|') | (2, 'F') => {
                                        if *search_pe == 'J'
                                            || *search_pe == '|'
                                            || *search_pe == 'L'
                                            || *search_pe == 'S'
                                        {
                                            self.output[state.row_index][state.col_index] =
                                                state.pe;
                                            return State {
                                                pe: *search_pe,
                                                distance: state.distance + 1,
                                                row_index: search_row_index,
                                                col_index: search_col_index,
                                                in_dir: Some(0),
                                            };
                                        }
                                    }
                                    (3, 'S') | (3, 'J') | (3, '-') | (3, '7') => {
                                        if *search_pe == 'L'
                                            || *search_pe == '-'
                                            || *search_pe == 'F'
                                            || *search_pe == 'S'
                                        {
                                            self.output[state.row_index][state.col_index] =
                                                state.pe;
                                            return State {
                                                pe: *search_pe,
                                                distance: state.distance + 1,
                                                row_index: search_row_index,
                                                col_index: search_col_index,
                                                in_dir: Some(1),
                                            };
                                        }
                                    }
                                    _ => {
                                        // self.output[search_row_index][search_col_index] =
                                        //     self.input[search_row_index][search_col_index];
                                    }
                                };
                            }
                        }
                    }
                }
            }
        }

        panic!("Walked an could not find the exit");
    }
}

fn part1(input: &str) -> u32 {
    let mut map: PipeMap = input.into();

    let mut state = State {
        pe: 'S',
        row_index: map.start_row_index,
        col_index: map.start_col_index,
        in_dir: None,
        distance: 0,
    };
    let mut lc = 0;
    'walking: loop {
        // Advance  walker until back at the start
        state = map.walk(state);

        if state.col_index == map.start_col_index && state.row_index == map.start_row_index {
            println!("match 1");
            break 'walking;
        }
        if state.pe == 'S' {
            println!("match2");
            break 'walking;
        }
        lc += 1;

        // break after n_tries
        if lc > 12_0000 {
            break;
        }
    }
    // dbg!(&first_walker);
    // assert_eq!(first_walker.distance, second_walker.distance);
    println!("{map}");
    state.distance / 2
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
            pe: 'S',
            row_index: map.start_row_index,
            col_index: map.start_col_index,
            in_dir: None,
        };

        let first_state = map.walk(start);
        // dbg!(&first_state);

        assert_eq!(
            first_state,
            State {
                pe: 'J',
                distance: 1,
                row_index: 2,
                col_index: 1,
                in_dir: Some(3)
            }
        );

        let second_path_state = map.walk(first_state);
        assert_eq!(
            second_path_state,
            State {
                pe: 'F',
                distance: 2,
                row_index: 1,
                col_index: 1,
                in_dir: Some(2)
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
            pe: 'S',
            distance: 0,
            row_index: map.start_row_index,
            col_index: map.start_col_index,
            in_dir: None,
        };

        let first_state = map.walk(start);

        assert_eq!(
            first_state,
            State {
                pe: '7',
                distance: 1,
                row_index: 2,
                col_index: 4,
                in_dir: Some(2)
            }
        );

        let dist1 = part1(lines);
        let dist2 = part1(lines);

        assert_eq!(dist1, 8);
        assert_eq!(dist2, 8);
    }

    #[test]
    fn example_square() {
        let lines = r".....
.S-7.
.|.|.
.L-J.
.....";

        assert_eq!(part1(lines), 4);
    }

    #[test]
    fn example_square_cluttered() {
        let lines = r"-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

        assert_eq!(part1(lines), 4);
    }

    fn complex_loop() {
        let lines = r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        assert_eq!(part1(lines), 4);
    }

    // Same patter as example1 but which clutter.
    fn complex_loop_with_clutter() {
        let lines = r"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

        assert_eq!(part1(lines), 4);
    }

    #[test]
    fn two_loops_unknown_length() {
        let lines = r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(part1(lines), 4);
    }

    #[test]
    fn two_loops_unknown_length2() {
        let lines = r"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

        assert_eq!(part1(lines), 3);
    }
}
