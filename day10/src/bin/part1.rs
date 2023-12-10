fn main() {
    let input = include_str!("./input1.txt");
    // println!("{:?}", part1(input));
}

// Map symbols including blank.
// pipe_elements = MAP_VALUE[0..=8]
const MAP_VALUE: [char; 9] = ['-', '7', '|', 'J', '-', 'L', '|', 'F', '.'];

// clock rotation: N, E, S, W
const DIRECTION: [(i16, i16); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Debug)]
struct PipeMap {
    row: Vec<Vec<char>>,
    start: (usize, usize),
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    x: usize,
    y: usize,
    blocked_direction: Option<usize>,
    distance: u16,
}

impl From<&str> for PipeMap {
    fn from(input: &str) -> Self {
        let mut start: (usize, usize) = (usize::MAX, usize::MAX);
        let row = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == 'S' {
                            start = (x, y);
                        }
                        c
                    })
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>();

        Self { row, start }
    }
}

impl PipeMap {
    fn walk(&self, state: State) -> State {
        // walk round the clock looking for the exit.
        let mut next_blocked_direction = state.blocked_direction;
        let mut next_x_index = i16::MAX;
        let mut next_y_index = i16::MAX;
        for (i, d) in DIRECTION.iter().enumerate() {
            if Some(i) != state.blocked_direction {
                next_x_index = state.x as i16 + d.0;
                if next_x_index >= 0 {
                    next_y_index = state.y as i16 + d.1;
                    if next_y_index >= 0 {
                        // The current pipe element under consideration.
                        let pe = self.row[next_y_index as usize][next_x_index as usize];
                        dbg!(next_x_index);
                        dbg!(next_y_index);
                        dbg!(pe);
                        // if we have found the exit and can compute the next
                        // backward direction
                        match i {
                            0 => {
                                if pe == '|' || pe == 'F' || pe == '7' {
                                    next_blocked_direction = Some(2);
                                    break;
                                }
                            }
                            1 => {
                                if pe == '-' || pe == '7' || pe == 'J' {
                                    next_blocked_direction = Some(3);
                                    break;
                                }
                            }
                            2 => {
                                if pe == '|' || pe == 'J' || pe == 'L' {
                                    next_blocked_direction = Some(0);
                                    break;
                                }
                            }
                            3 => {
                                if pe == '-' || pe == 'F' || pe == 'L' {
                                    next_blocked_direction = Some(1);
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
        let new_state = State {
            distance: state.distance + 1,
            x: next_x_index as usize,
            y: next_y_index as usize,
            blocked_direction: next_blocked_direction,
        };
        new_state
    }
}

// fn part1(input: &str) -> u32 {
//         let map : PipeMap = input.into();

// }

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
        let map: PipeMap = lines.into();
        let start_pos = map.start;
        assert_eq!(start_pos, (0, 2));
        let start = State {
            distance: 0,
            x: start_pos.0,
            y: start_pos.1,
            blocked_direction: None,
        };
        let first_path_state = map.walk(start);
        dbg!(&first_path_state);

        assert_eq!(
            first_path_state,
            State {
                distance: 1,
                x: 1,
                y: 2,
                blocked_direction: Some(3)
            }
        );

        let second_start = State {
            distance: 0,
            x: start_pos.0,
            y: start_pos.1,
            blocked_direction: Some(1),
        };
        let second_path_state = map.walk(second_start);
        assert_eq!(
            second_path_state,
            State {
                distance: 1,
                x: 0,
                y: 3,
                blocked_direction: Some(0)
            }
        );
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
