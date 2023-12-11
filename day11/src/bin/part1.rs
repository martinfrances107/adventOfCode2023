use core::fmt::Debug;
use core::fmt::Display;

use std::collections::VecDeque;

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

#[derive(Copy, Clone, Debug, Eq)]
struct Galaxy {
    id: u64,
    row_index: usize,
    col_index: usize,
}

impl Galaxy {
    fn new(id: u64, row_index: usize, col_index: usize) -> Self {
        Galaxy {
            id,
            row_index,
            col_index,
        }
    }
}
impl PartialEq for Galaxy {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Cell {
    Blank,
    Galaxy(Galaxy),
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
            .enumerate()
            .map(|(row_index, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col_index, c)| {
                        if c == '.' {
                            Cell::Blank
                        } else if c == '#' {
                            start_id += 1;
                            Cell::Galaxy(Galaxy::new(start_id, row_index, col_index))
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
        let mut rows: Vec<Vec<Cell>> = Vec::with_capacity(n_rows);
        for (row_index, row) in self.rows.iter().enumerate() {
            let mut new_row: Vec<Cell> = Vec::with_capacity(n_rows);
            for (col_index, cell) in row.iter().enumerate() {
                new_row.push(*cell);
                if blank_cols.contains(&col_index) {
                    new_row.push(Cell::Blank);
                }
            }
            if blank_rows.contains(&row_index) {
                rows.push(new_row.clone());
                rows.push(new_row);
            } else {
                rows.push(new_row);
            }
        }

        Self { rows }
    }

    // Return list of galaxies found in the map.
    fn get_galaxy_list(&self) -> Vec<Galaxy> {
        let galaxy_list = self
            .rows
            .iter()
            .flat_map(|row| {
                row.iter()
                    .filter_map(|cell| match cell {
                        Cell::Blank => None,
                        Cell::Galaxy(galaxy) => Some(*galaxy),
                    })
                    .collect::<Vec<Galaxy>>()
            })
            .collect::<Vec<Galaxy>>();
        galaxy_list
    }

    fn compute_parings(g_list: &Vec<Galaxy>) -> Vec<(Galaxy, Galaxy)> {
        let mut g_list: VecDeque<Galaxy> = (*g_list).clone().into();
        let mut pairings: Vec<(Galaxy, Galaxy)> = vec![];

        let mut current = g_list.pop_front().unwrap();
        loop {
            for item in &g_list {
                pairings.push((current, *item));
            }

            if let Some(n) = g_list.pop_front() {
                current = n;
            } else {
                break;
            }
        }
        pairings
    }

    fn compute_manhatten_distance(a: &Galaxy, b: &Galaxy) -> i128 {
        let h = (a.row_index as i128 - b.row_index as i128).abs();
        let v = (a.col_index as i128 - b.col_index as i128).abs();
        h + v
    }

    fn compute_min_distances(&self) -> Vec<i128> {
        let gl = self.get_galaxy_list();
        let pairings = StarMap::compute_parings(&gl);

        let min_distances = pairings
            .iter()
            .map(|(g0, g1)| StarMap::compute_manhatten_distance(g0, g1))
            .collect::<Vec<_>>();

        min_distances
    }
}

fn part1(input: &str) -> i128 {
    let map: StarMap = input.into();
    let pre_min_distances: i128 = map.compute_min_distances().iter().sum();
    dbg!(pre_min_distances);
    let blank_rows = map.collect_blank_rows();
    let blank_cols = map.collect_blank_cols();
    dbg!(&blank_rows);
    dbg!(&blank_cols);
    let expanded_map = map.expand(&blank_rows, &blank_cols);
    expanded_map.compute_min_distances().iter().sum()
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
        assert_eq!(map.get_galaxy_list().len(), 9,);

        let gl = map.get_galaxy_list();
        let out = StarMap::compute_parings(&gl);
        assert_eq!(out.len(), 36);
    }

    #[test]
    fn test_manhatten() {
        let input = r"....#........
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
        let map: StarMap = input.into();

        let gl = map.get_galaxy_list();
        let pairings = StarMap::compute_parings(&gl);
        let (g1, g7) = pairings
            .iter()
            .find(|pair| {
                let (g0, g1) = **pair;
                g0.id == 1 && g1.id == 7
            })
            .unwrap();
        let d_g1_g7 = StarMap::compute_manhatten_distance(g1, g7);
        assert_eq!(d_g1_g7, 15);

        let (g3, g6) = pairings
            .iter()
            .find(|pair| {
                let (g0, g1) = **pair;
                g0.id == 3 && g1.id == 6
            })
            .unwrap();
        let d_g3_g6 = StarMap::compute_manhatten_distance(g3, g6);
        assert_eq!(d_g3_g6, 17);

        let (g8, g9) = pairings
            .iter()
            .find(|pair| {
                let (g0, g1) = **pair;
                g0.id == 8 && g1.id == 9
            })
            .unwrap();
        let d_g8_g9 = StarMap::compute_manhatten_distance(g9, g8);
        assert_eq!(d_g8_g9, 5);
    }

    #[test]
    fn test_manhatten_combined() {
        let input = r"....#........
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

        let map: StarMap = input.into();

        let min_d: i128 = map.compute_min_distances().iter().sum();
        assert_eq!(min_d, 374i128);
    }
}
