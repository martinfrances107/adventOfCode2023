struct AshMirror {
    col_map: Vec<Vec<char>>,
    row_map: Vec<Vec<char>>,
}

impl AshMirror {
    fn new(input: &str) -> Self {
        let row_map = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();

        let col_map = col_map(row_map.clone());

        Self { col_map, row_map }
    }

    fn from_puzzle(input: &[&str]) -> Self {
        let row_map = input
            .iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();

        let col_map = col_map(row_map.clone());

        Self { col_map, row_map }
    }

    fn score(&self) -> u128 {
        let mut score: u128 = 0;
        let candidate_row_points = row_matches(&self.row_map);
        let h_mps: Vec<_> = candidate_row_points
            .iter()
            .filter_map(|mp| {
                if is_a_horizontal_mirror_point(*mp, self.row_map.clone()) {
                    Some(mp)
                } else {
                    None
                }
            })
            .collect();

        match h_mps.len() {
            0 => {}
            1 => score += h_mps[0].0 * 100,
            _ => {
                panic!("unexpected number of h. mirror points");
            }
        }

        let candidate_col_points = col_matches(&self.col_map);
        let v_mps: Vec<_> = candidate_col_points
            .iter()
            .filter_map(|mp| {
                if is_a_veritcal_mirror_point(*mp, self.col_map.clone()) {
                    Some(mp)
                } else {
                    None
                }
            })
            .collect();
        match v_mps.len() {
            0 => {}
            1 => score += v_mps[0].0,
            _ => {
                panic!("unexpected number of v. mirror points");
            }
        }
        score
    }
}

fn main() {
    let input = include_str!("./input1.txt");
    // println!("{:?}", part1(input));

    let mut total = 0u128;
    let lines: Vec<_> = input.lines().map(|line| line).collect();

    let puzzles = lines
        .split(|line| {
            // a
            line.is_empty()
        })
        .collect::<Vec<_>>();

    for p in puzzles {
        println!("new Puzzle");
        for l in p {
            println!("{l}");
        }
        let am = AshMirror::from_puzzle(p);
        let p_score = am.score();
        total += p_score;
        println!("score: {p_score}");
        println!();
    }
    println!("34103 is too low");
    println!("Total: {total}");
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

fn part1() -> u32 {
    todo!();
}

fn row_matches(row_map: &Vec<Vec<char>>) -> Vec<(u128, u128)> {
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

fn col_matches(col_map: &Vec<Vec<char>>) -> Vec<(u128, u128)> {
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

fn is_a_horizontal_mirror_point(reflection_edge: (u128, u128), hor_map: Vec<Vec<char>>) -> bool {
    // for each advancing left-hand-edge
    // compare with the corresponding right-hand-edge.

    // convert_back to index
    let left_index = (reflection_edge.0 - 1) as usize;
    let right_index = (reflection_edge.1 - 1) as usize;

    dbg!(left_index);
    dbg!(right_index);

    if left_index == 0 {
        return false;
    }
    // reflection_edge has been checked, hence the -1 and +1 below.
    let left_walker = 0..=left_index - 1;
    let right_walker = right_index + 1..hor_map.len();

    let breaks_pattern = left_walker
        .into_iter()
        .rev()
        .zip(right_walker)
        .any(|(l, r)| hor_map[l] != hor_map[r]);

    !breaks_pattern
}

fn is_a_veritcal_mirror_point(reflection_edge: (u128, u128), col_map: Vec<Vec<char>>) -> bool {
    // for each advancing left-hand-edge
    // compare with the corresponding right-hand-edge.

    // convert_back to index
    let left_index = (reflection_edge.0 - 1) as usize;
    let right_index = (reflection_edge.1 - 1) as usize;

    dbg!(left_index);
    dbg!(right_index);

    if left_index == 0 {
        return false;
    }
    // reflection_edge has been checked, hence the -1 and +1 below.
    let left_walker = 0..=left_index - 1;
    let right_walker = right_index + 1..col_map.len();

    let breaks_pattern = left_walker
        .into_iter()
        .rev()
        .zip(right_walker)
        .any(|(l, r)| match (col_map.get(l), col_map.get(r)) {
            (Some(l_col), Some(r_col)) => l_col != r_col,
            _ => true,
        });

    !breaks_pattern
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

        let candidate_mpoints = row_matches(&row_map);
        assert_eq!(candidate_mpoints, vec![(4, 5)]);
        assert!(is_a_horizontal_mirror_point(candidate_mpoints[0], row_map),);
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
        let col_map = col_map(row_map.clone());
        let candidate_mpoints = col_matches(&col_map);
        assert_eq!(candidate_mpoints, vec![(5, 6)]);
        assert!(is_a_veritcal_mirror_point(candidate_mpoints[0], col_map),);
    }

    #[test]
    fn not_a_true_vertical_mirror() {
        // Pattern is broken on the second to last row.
        let broken_pattern: &str = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##...#.
#.#.##.#.";
        let row_map = broken_pattern
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let col_map = col_map(row_map.clone());
        let candidate_mpoints = col_matches(&col_map);
        assert_eq!(candidate_mpoints, vec![(5, 6)]);
        assert!(!is_a_veritcal_mirror_point(candidate_mpoints[0], col_map));
    }

    #[test]
    fn not_a_true_horizontal_mirror() {
        // Pattern is broken on the second row
        let broken_pattern: &str = r"#...##..#
#....#..#
..##..##.
#####.##.
#####.##.
..##..###
#....#..#";
        let row_map = broken_pattern
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let candidate_mpoints = row_matches(&row_map);
        assert_eq!(candidate_mpoints, vec![(4, 5)]);
        assert!(!is_a_horizontal_mirror_point(candidate_mpoints[0], row_map));
    }

    #[test]
    fn score_h() {
        let input = r"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let mirror = AshMirror::new(input);
        assert_eq!(mirror.score(), 400);
    }

    #[test]
    fn score_vertical() {
        let input = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let mirror = AshMirror::new(input);
        assert_eq!(mirror.score(), 5);
    }
}
