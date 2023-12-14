fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

#[derive(Debug, PartialEq, Eq)]
struct ShuffleBoard {
    col_map: Vec<Vec<char>>,
    row_map: Vec<Vec<char>>,
}

impl ShuffleBoard {
    fn new(input: &str) -> Self {
        let row_map = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();

        let col_map = col_map(&row_map.clone());

        Self { col_map, row_map }
    }

    fn score(&self) -> u64 {
        let len = self.row_map.len();
        self.row_map
            .iter()
            .enumerate()
            .map(|(row_index, row)| {
                // number of rocks
                let n_rocks: u64 = row.iter().map(|c| if *c == 'O' { 1 } else { 0 }).sum();
                let col_number: u64 = (len - row_index) as u64;
                n_rocks * col_number
            })
            .sum()
    }

    fn slide_col(col: &mut [char]) {
        // split into movement sections

        loop {
            let slide_list = col
                .windows(2)
                .enumerate()
                .filter_map(|(index, pair)| {
                    debug_assert!(pair.len() == 2);
                    let l = *pair
                        .get(0)
                        //"window has two elements cannot locate first"
                        .unwrap();
                    let r = *pair
                        .get(1)
                        // "window has two elements cannot locate second"
                        .unwrap();
                    // dbg!(pair);
                    if (l, r) == ('.', 'O') {
                        Some(index)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            dbg!(&slide_list);
            if slide_list.is_empty() {
                break;
            }

            for left_swap in slide_list {
                col.swap(left_swap, left_swap + 1);
            }
        }
        dbg!("final");
        dbg!(&col);
    }

    fn slide(&mut self) {
        // Iterator over every column sliding.
        for col in self.col_map.iter_mut() {
            Self::slide_col(col)
        }
    }
}

fn part1(input: &str) -> u64 {
    let mut sb = ShuffleBoard::new(input);
    sb.slide();
    sb.score()
}

fn col_map(row_map: &[Vec<char>]) -> Vec<Vec<char>> {
    let row_len = row_map[0].len();
    // let mut col_map: Vec<Vec<char>> = Vec::with_capacity(row_len);
    // Every column needs to be defined before population.
    let mut col_map: Vec<Vec<char>> = (1..=row_len).map(|_| vec![]).collect();
    for row in row_map.iter() {
        for (col_index, c) in (*row).iter().enumerate() {
            col_map[col_index].push(*c);
        }
    }
    col_map
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn score() {
        let shuffled = r"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

        let sb = ShuffleBoard::new(shuffled);
        assert_eq!(sb.score(), 136);
    }

    #[test]
    fn single_slder() {
        let input = "O
O
.
O
.
O
.
.
#
#";
        let mut actual = ShuffleBoard::new(input);
        actual.slide();

        // epected expressed a a file input.
        let expected_input = "O
O
O
O
.
.
.
.
#
#";
        let expected = ShuffleBoard::new(expected_input);

        assert_eq!(actual.col_map, expected.col_map);
    }

    #[test]
    fn complete_slider() {
        let initial = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        let mut sb = ShuffleBoard::new(initial);
        sb.slide();

        let f = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

        let expected = ShuffleBoard::new(f);
        assert_eq!(sb.col_map, expected.col_map);
    }
}
