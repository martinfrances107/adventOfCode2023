fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

#[derive(Debug, Default)]
struct Balls {
    red: u32,
    green: u32,
    blue: u32,
}

impl Balls {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_game_header, sets_line) = line
                .split_once(':')
                .expect("well formatted code: There will be at least one ':'");

            let sets = sets_line.split(';');

            // A game is a sequence of sets.
            // Convert each set in a ball struct
            // check for excess, report if possible
            let mut max_b = Balls::default();
            for set in sets {
                let values: Vec<&str> = set.split(',').collect();
                for value in values {
                    if let Some(n_str) = value.strip_suffix(&"red") {
                        let n = n_str
                            .trim()
                            .parse::<u32>()
                            .expect("what remains MUST be a number");
                        if n > max_b.red {
                            max_b.red = n;
                        }
                    }
                    if let Some(n_str) = value.strip_suffix(&"green") {
                        let n = n_str
                            .trim()
                            .parse::<u32>()
                            .expect("what remains MUST be a number");
                        if n > max_b.green {
                            max_b.green = n;
                        }
                    }
                    if let Some(n_str) = value.strip_suffix(&"blue") {
                        let n = n_str
                            .trim()
                            .parse::<u32>()
                            .expect("what remains MUST be a number");
                        if n > max_b.blue {
                            max_b.blue = n;
                        }
                    }
                }
            }

            //The power of the max seen solution
            max_b.power()
        })
        .sum()
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_part2() {
        let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part1(input), 2286u32)
    }
}
