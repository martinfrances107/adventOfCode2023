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
    fn is_exceeded(&self, rhs: &Self) -> bool {
        rhs.red > self.red || rhs.green > self.green || rhs.blue > self.blue
    }
}

const BALL_THRESHOLD: Balls = Balls {
    red: 12u32,
    green: 13u32,
    blue: 14u32,
};

fn part1(input: &str) -> u32 {
    let out = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let game_id = i as u32 + 1;

            let (_game_header, sets_line) = line
                .split_once(':')
                .expect("well formatted code: There will be at least one ':'");

            let sets = sets_line.split(';');

            // Game is a "possible", no fails.
            let mut is_possible = true;

            // A game is a sequence of sets.
            // Convert each set in a ball struct
            // check for excess, report if possible
            for set in sets {
                let values: Vec<&str> = set.split(',').collect();
                let mut b = Balls::default();
                for value in values {
                    if let Some(n_str) = value.strip_suffix(&"red") {
                        let n = n_str
                            .trim()
                            .parse::<u32>()
                            .expect("what remains MUST be a number");
                        b.red += n;
                    }
                    if let Some(n_str) = value.strip_suffix(&"green") {
                        let n = n_str
                            .trim()
                            .parse::<u32>()
                            .expect("what remains MUST be a number");
                        b.green += n;
                    }
                    if let Some(n_str) = value.strip_suffix(&"blue") {
                        let n = n_str
                            .trim()
                            .parse::<u32>()
                            .expect("what remains MUST be a number");
                        b.blue += n;
                    }

                    if BALL_THRESHOLD.is_exceeded(&b) {
                        is_possible = false;
                    }
                }
            }

            // Return the game_id if none of the game sets where exceeded.
            if is_possible {
                game_id
            } else {
                0u32
            }
        })
        .collect::<Vec<u32>>();

    dbg!(&out);
    out.iter().sum()
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_part1() {
        let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part1(input), 8u32)
    }
}
