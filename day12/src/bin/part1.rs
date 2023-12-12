use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

lazy_static! {
   // line is separated in thee block FIRST, MID, LAST
    static ref FIRST_BLOCK: Regex = Regex::new(r"^O*(?<FIRST_BLOCK>D?M*D?)O").unwrap();
    static ref LAST_BLOCK: Regex = Regex::new(r"O(?<LAST_BLOCK>D*|D?M*D?|)O*$").unwrap();
}

fn transform(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            // Damage
            '#' => 'D',
            // Operational
            '.' => 'O',
            // Maybe
            '?' => 'M',
            _ => {
                panic!("cannot transform unexpected character")
            }
        })
        .collect::<String>()
}

fn part1(input: &str) -> u32 {
    let a = input
        .lines()
        .map(|line_raw| {
            let line = transform(line_raw);
            // dbg!(RE.captures(&line));
            1
        })
        .collect::<Vec<_>>();
    todo!();
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn mutate() {
        let line = r".?#";
        assert_eq!(transform(&line), String::from("OMD"));
    }

    #[test]
    fn simple() {
        let test_cases: [(&str, (&str, &str)); 1] = [("???.###", ("MMM", "DDD"))];
        for tc in test_cases {
            let line = transform(&tc.0);
            let first_captures = FIRST_BLOCK.captures(&line);
            let fb = first_captures.unwrap().name("FIRST_BLOCK").unwrap();

            let last_captures = LAST_BLOCK.captures(&line);
            let lb = last_captures.unwrap().name("LAST_BLOCK").unwrap();

            let expected = tc.1;

            assert_eq!(fb.as_str(), expected.0);
            assert_eq!(lb.as_str(), expected.1);
        }
    }
}
