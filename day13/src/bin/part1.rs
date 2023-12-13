use core::iter::successors;

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn col_map(row_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
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
fn part1(input: &str) -> u32 {
    let row_map = input.lines().map(|line| line).collect::<Vec<_>>();

    // File char by colummn

    // let col_map
    todo!();
}

fn row_matches(row_map: Vec<Vec<char>>) -> Vec<(u128, u128)> {
    row_map
        .windows(2)
        .enumerate()
        .filter_map(|(row_index, row_pair)| {
            if *row_pair[0] == *row_pair[1] {
                // convert col_index into column number
                // index: zero based counting
                Some((row_index as u128 + 1, (row_index + 2) as u128))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn col_matches(col_map: Vec<Vec<char>>) -> Vec<(u128, u128)> {
    col_map
        .windows(2)
        .enumerate()
        .filter_map(|(col_index, col_pair)| {
            if *col_pair[0] == *col_pair[1] {
                // convert col_index into column number
                // index: zero based counting
                // col number starts at one.
                Some(((col_index + 1) as u128, (col_index + 2) as u128))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn horizontal() {
        let input = r"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let row_map = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();

        assert_eq!(row_matches(row_map), vec![(4, 5)])
    }

    #[test]
    fn vertical() {
        let input = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let row_map = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let col_map = col_map(row_map);
        assert_eq!(col_matches(col_map), vec![(5, 6)]);
    }
}
