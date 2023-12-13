fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn col_map(row_map: Vec<&str>) -> Vec<Vec<char>> {
    let row_len = row_map[0].len();
    let mut col_map: Vec<Vec<char>> = Vec::with_capacity(row_len);
    for row in row_map.iter() {
        for (col_index, c) in (*row).chars().enumerate() {
            col_map[col_index].push(c);
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

fn row_matches(row_map: &Vec<&str>) -> Vec<(usize, usize)> {
    todo!();
}

fn col_matches(col_map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    todo!();
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn horizontal() {
        let input = r"#.##..##.
      ..#.##.#.
      ##......#
      ##......#
      ..#.##.#.
      ..##..##.
      #.#.##.#.";
        let row_map = input.lines().map(|line| line).collect::<Vec<_>>();
        assert_eq!(row_matches(&row_map), vec![(4, 5)])
    }

    #[test]
    fn vertical() {
        let input = r"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let row_map = input.lines().map(|line| line).collect::<Vec<_>>();
        let col_map = col_map(row_map);
        assert_eq!(col_matches(&col_map), vec![(5, 6)]);
    }
}
