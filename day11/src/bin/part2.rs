use core::fmt::Debug;
use core::fmt::Display;

use std::collections::VecDeque;

fn main() {
    let input = include_str!("./sample.txt");
    println!("{:?}", part2(input, 1000_000));
}

#[derive(Copy, Clone, Debug, Eq)]
struct Galaxy {
    id: u64,
    row_index: usize,
    col_index: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Space {
    v_length: usize,
    h_length: usize,
}

// The default blank space is 1x1 box
// so we must override the usize default.
impl Default for Space {
    fn default() -> Self {
        Self {
            v_length: 1usize,
            h_length: 1usize,
        }
    }
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
    Blank(Space),
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
                match cell {
                    Cell::Blank(Space { v_length, h_length }) => {
                        write!(f, " ({h_length} - {v_length}) ")?;
                    }
                    Cell::Galaxy(..) => {
                        write!(f, " (1 - 1)")?;
                    }
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
                match cell {
                    Cell::Blank(..) => {
                        write!(f, ".")?;
                    }
                    Cell::Galaxy(..) => {
                        write!(f, "#")?;
                    }
                }
            }
            writeln!(f)?;
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
                            Cell::Blank(Space::default())
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
                    Cell::Blank(_) => false,
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
                    Cell::Blank(_) => {}
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

    fn expand(self, bump: usize, blank_rows: &[usize], blank_cols: &[usize]) -> Self {
        dbg!("expanding");
        let n_rows = self.rows.len();
        let mut rows: Vec<Vec<Cell>> = Vec::with_capacity(n_rows);
        for (row_index, row) in self.rows.iter().enumerate() {
            let mut new_row: Vec<Cell> = Vec::with_capacity(n_rows);
            for (col_index, cell) in row.iter().enumerate() {
                if blank_cols.contains(&col_index) {
                    // Stretch horizontally.
                    match cell {
                        Cell::Blank(Space {
                            v_length,
                            h_length: _,
                        }) => new_row.push(Cell::Blank(Space {
                            v_length: *v_length,
                            h_length: bump,
                        })),
                        Cell::Galaxy(_) => {
                            panic!("found a galaxy on the blank colummn");
                        }
                    }
                } else {
                    // Just copy
                    new_row.push(*cell);
                }
            }
            if blank_rows.contains(&row_index) {
                // loop over every element in the horizontal row
                // stretching vertically.
                for cell in new_row.iter_mut() {
                    match cell {
                        Cell::Blank(Space {
                            v_length,
                            h_length: _,
                        }) => *v_length = bump,
                        Cell::Galaxy(..) => {
                            panic!("blank row cannot contain a galaxy");
                        }
                    }
                }
            }
            rows.push(new_row);
        }

        // The galaxies have shifted recompute positions
        // TODO double looping, can I loop just once?
        for (new_row_index, row) in rows.iter_mut().enumerate() {
            for (new_col_index, cell) in row.iter_mut().enumerate() {
                match cell {
                    Cell::Galaxy(Galaxy {
                        id: _id,
                        row_index,
                        col_index,
                    }) => {
                        *row_index = new_row_index;
                        *col_index = new_col_index;
                    }
                    Cell::Blank(..) => {}
                }
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
                        Cell::Blank(..) => None,
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

    fn compute_manhatten_distance(&self, a: &Galaxy, b: &Galaxy) -> u128 {
        // walk horizontally along a's col adding up variable lengths.
        // walk vertically along b column add up lengths.
        let row = self.rows[a.row_index].clone();
        let mut h_dist = 0;
        for i in a.row_index + 1..=b.row_index {
            h_dist += match row[i] {
                Cell::Blank(Space {
                    h_length,
                    v_length: _,
                }) => h_length as u128,
                Cell::Galaxy(..) => 1,
            }
        }

        // walk b column
        let mut v_dist = 0;
        for i in a.row_index + 1..=b.row_index {
            v_dist += match self.rows[i][b.col_index] {
                Cell::Blank(Space {
                    h_length: _,
                    v_length,
                }) => v_length as u128,
                Cell::Galaxy(..) => 1,
            }
        }
        v_dist + h_dist
    }

    fn compute_min_distances(&self) -> Vec<u128> {
        let gl = self.get_galaxy_list();
        let pairings = StarMap::compute_parings(&gl);

        let min_distances = pairings
            .iter()
            .map(|(g0, g1)| self.compute_manhatten_distance(g0, g1))
            .collect::<Vec<_>>();

        min_distances
    }
}

fn part2(input: &str, bump: usize) -> u128 {
    let map: StarMap = input.into();

    print!("{map:#?}");
    let blank_rows = map.collect_blank_rows();
    let blank_cols = map.collect_blank_cols();

    let expanded_map = map.expand(bump, &blank_rows, &blank_cols);
    print!("{expanded_map:#?}");
    let mut sum = 0;
    for d in expanded_map.compute_min_distances() {
        sum += d;
    }

    sum
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
    fn sparse_distances() {
        let input = r"#...
...
..#";

        assert_eq!(part2(input, 10), 22);
        assert_eq!(part2(input, 100), 201);
        assert_eq!(part2(input, 1_000_000), 2_000_002);
    }

    #[test]
    fn no_blank_col() {
        let no_blanks = r"#########";
        let no_blanks_map: StarMap = no_blanks.into();
        let blank_rows = no_blanks_map.collect_blank_rows();
        let blank_cols = no_blanks_map.collect_blank_cols();
        dbg!(&blank_rows);
        dbg!(&blank_cols);
        let expanded_map = no_blanks_map.clone().expand(10, &blank_rows, &blank_cols);

        assert_eq!(no_blanks_map, expanded_map);
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
        let gl = map.get_galaxy_list();
        assert_eq!(gl.len(), 9,);

        let out = StarMap::compute_parings(&gl);
        assert_eq!(out.len(), 36);
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

        assert_eq!(part2(input, 10), 1030);
        // assert_eq!(part2(input, 100), 8410);
    }
}
