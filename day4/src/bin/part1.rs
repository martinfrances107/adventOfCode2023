fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut score = 0_u32;
            // w_p  winners and player numbers.
            if let Some((_header, w_p)) = line.split_once(':') {
                if let Some((w, p)) = w_p.split_once('|') {
                    let w_numbers = w
                        .split(' ')
                        .filter_map(|s| s.parse::<u32>().ok())
                        .collect::<Vec<u32>>();
                    let p_numbers = p
                        .split(' ')
                        .filter_map(|s| s.parse::<u32>().ok())
                        .collect::<Vec<u32>>();

                    let mut x = 0;
                    for w in &w_numbers {
                        for p in &p_numbers {
                            if *w == *p {
                                x += 1;
                            }
                        }
                    }

                    if x != 0 {
                        score = 1u32 << (x - 1)
                    }
                }
            }
            score
        })
        .sum()
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn example() {
        let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part1(input), 13u32)
    }
}
