use core::fmt::Debug;
use core::fmt::Display;

use std::collections::VecDeque;

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Cell {
    Blank,
    Galaxy(u64),
}

#[derive(Clone, PartialEq, Eq)]
struct StarMap {
    rows: Vec<Vec<Cell>>,
}

impl Debug for StarMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in &self.rows {
            for cell in row {
                if *cell == Cell::Blank {
                    write!(f, ".")?;
                } else {
                    write!(f, "#")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for StarMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for cell in row {
                if *cell == Cell::Blank {
                    write!(f, ".")?;
                } else {
                    write!(f, "#")?;
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
impl From<&str> for StarMap {
    fn from(input: &str) -> Self {
        // The next start id is zero;
        let mut start_id = 0u64;
        let rows = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        if c == '.' {
                            Cell::Blank
                        } else if c == '#' {
                            start_id += 1;
                            Cell::Galaxy(start_id)
                        } else {
                            panic!("malfomed map.");
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>();
        Self { rows }
    }
}

impl StarMap {
    fn collect_blank_rows(&self) -> Vec<usize> {
        let b_rows = self
            .rows
            .iter()
            .enumerate()
            .filter_map(|(row_id, row)| {
                if row.iter().any(|cell| match *cell {
                    Cell::Galaxy(_) => true,
                    Cell::Blank => false,
                }) {
                    None
                } else {
                    Some(row_id)
                }
            })
            .collect::<Vec<usize>>();

        b_rows
    }

    fn collect_blank_cols(&self) -> Vec<usize> {
        let mut col_count: Vec<u32> = vec![0; self.rows[0].len()];

        for row in self.rows.iter() {
            for (col_index, cell) in row.iter().enumerate() {
                match *cell {
                    Cell::Galaxy(_) => {
                        col_count[col_index] += 1;
                    }
                    Cell::Blank => {}
                }
            }
        }

        col_count
            .iter()
            .enumerate()
            .filter_map(|(col_index, count)| {
                if *count == 0u32 {
                    Some(col_index)
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>()
    }

    fn expand(self, blank_rows: &[usize], blank_cols: &[usize]) -> Self {
        let n_rows = self.rows.len();
        let mut expanded_map: Vec<Vec<Cell>> = Vec::with_capacity(n_rows);
        for (row_index, row) in self.rows.iter().enumerate() {
            let mut new_row: Vec<Cell> = Vec::with_capacity(n_rows);
            for (col_index, cell) in row.iter().enumerate() {
                new_row.push(*cell);
                if blank_cols.contains(&col_index) {
                    new_row.push(Cell::Blank);
                }
            }
            if blank_rows.contains(&row_index) {
                expanded_map.push(new_row.clone());
                expanded_map.push(new_row);
            } else {
                expanded_map.push(new_row);
            }
        }

        Self { rows: expanded_map }
    }

    fn get_galaxy_list(&self) -> Vec<Cell> {
        // find galaxies
        let galaxy_list = self
            .rows
            .iter()
            .flat_map(|row| {
                row.iter()
                    .filter_map(|cell| match cell {
                        Cell::Blank => None,
                        Cell::Galaxy(id) => Some(Cell::Galaxy(*id)),
                    })
                    .collect::<Vec<Cell>>()
            })
            .collect::<Vec<Cell>>();
        galaxy_list
    }

    fn compute_parings(g_list: &Vec<Cell>) -> Vec<(Cell, Cell)> {
        let mut g_list: VecDeque<Cell> = (*g_list).clone().into();
        let mut pairings: Vec<(Cell, Cell)> = vec![];

        let mut current = g_list.pop_front().unwrap();
        loop {
            for item in &g_list {
                pairings.push((current, *item));
            }
            dbg!(&pairings);
            if let Some(n) = g_list.pop_front() {
                current = n;
            } else {
                break;
            }
        }
        pairings
    }
}

fn part1(input: &str) -> u32 {
    let map: StarMap = input.into();
    let blank_rows = map.collect_blank_rows();
    let blank_cols = map.collect_blank_cols();
    let _expanded_map = map.expand(&blank_rows, &blank_cols);

    todo!();
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_blank_rows_cols() {
        let input = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let map: StarMap = input.into();

        assert_eq!(map.collect_blank_rows(), vec![3, 7]);

        assert_eq!(map.collect_blank_cols(), vec![2, 5, 8])
    }

    #[test]
    fn no_blank_col() {
        let no_blanks = r"#########";
        let no_blanks_map: StarMap = no_blanks.into();
        let blank_rows = no_blanks_map.collect_blank_rows();
        let blank_cols = no_blanks_map.collect_blank_cols();
        dbg!(&blank_rows);
        dbg!(&blank_cols);
        let expanded_map = no_blanks_map.clone().expand(&blank_rows, &blank_cols);

        assert_eq!(no_blanks_map, expanded_map);
    }
    #[test]
    fn one_blank_col() {
        let one_blank = r"####.####";
        let one_blank_map: StarMap = one_blank.into();
        let blank_rows = one_blank_map.collect_blank_rows();
        let blank_cols = one_blank_map.collect_blank_cols();
        dbg!(&blank_rows);
        dbg!(&blank_cols);
        let expanded_map = one_blank_map.expand(&blank_rows, &blank_cols);

        let expected_line = r"####..####";
        let expected = expected_line.into();
        assert_eq!(expanded_map, expected);
    }

    #[test]
    fn text_expanded_example() {
        let input = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let map: StarMap = input.into();

        let blank_rows = map.collect_blank_rows();
        let blank_cols = map.collect_blank_cols();
        let expanded_map = map.expand(&blank_rows, &blank_cols);

        let expected_input = r"....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";

        let expected_map = expected_input.into();

        assert_eq!(expanded_map, expected_map);
    }

    #[test]
    fn test_galaxy_pairings() {
        let input = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let map: StarMap = input.into();
        assert_eq!(
            map.get_galaxy_list(),
            vec![
                Cell::Galaxy(1),
                Cell::Galaxy(2),
                Cell::Galaxy(3),
                Cell::Galaxy(4),
                Cell::Galaxy(5),
                Cell::Galaxy(6),
                Cell::Galaxy(7),
                Cell::Galaxy(8),
                Cell::Galaxy(9)
            ]
        );

        let gl = map.get_galaxy_list();
        let out = StarMap::compute_parings(&gl);
        assert_eq!(out.len(), 36);
    }
}