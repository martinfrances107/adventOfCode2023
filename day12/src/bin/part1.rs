


fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

// lazy_static! {
//    // line is separated in thee block FIRST, MID, LAST
//     static ref FIRST_BLOCK: Regex = Regex::new(r"^O*(?<FIRST_BLOCK>D?M*D?)O").unwrap();
//     static ref LAST_BLOCK: Regex = Regex::new(r"O(?<LAST_BLOCK>D*M+D*|D+)O*$").unwrap();
// }

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

fn mutate() {
    let line = r".?#";
    assert_eq!(transform(line), String::from("OMD"));
}

// Strip off leading '.' and strip off tailing '.'
// TODO Could use skip_while
fn limit_search_area(line: &str) -> &str {
    // strip offf leading and trailing '.'
    let start_pos = line.chars().position(|c| c != '.').unwrap();
    let steps_from_end = line.chars().rev().position(|c| c != '.').unwrap();
    &line[start_pos..line.len() - steps_from_end]
}

struct FittingError;

#[derive(Debug, PartialEq, Eq)]
enum Fit {
    // if all the ? are converted into # this will be a valid solution.
    Yes,
    // It depends, if the terminating char is a . and all the conversion from
    // '?' to '#' procedd then this is a valid solution.
    Maybe,
    // Overrun the run of # and ? is too long for a valid fit.
    No,
}

fn possible_fit(line: &str, length: usize) -> Fit {
    // Grab while valid upto the prescribed length.
    let run_length = line
        .chars()
        .take(length)
        .take_while(|c| {
            dbg!("take");
            *c == '#' || *c == '?'
        })
        .count();

    // Is the next char a valid terminating node.
    match line.chars().nth(run_length) {
        Some(c) => match c {
            '?' => Fit::Maybe,
            '.' => Fit::Yes,
            '#' => Fit::No,
            _ => panic!("bad char"),
        },
        // Overrun is a good thing.
        // It is a subsitute for a terminating node
        None => Fit::Yes,
    }
}

fn part1(_input: &str) -> u32 {
    todo!();
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn top_and_tail() {
        assert_eq!(limit_search_area(r"...#.###??.?..."), "#.###??.?");
    }

    // test cases with all leading and trailing '.' removed.
    #[test]
    fn fitting() {
        let testcases = vec![
            (r"?#", Fit::No),
            // Maybe the inards and possible  AND terminating is possibly correct.
            (r"??", Fit::Maybe),
            // The inards and definite, terminating is possibly correct.
            (r"#?", Fit::Maybe),
            // Maybe the inards and definite, overuning here is a positive thing.
            (r"?", Fit::Yes),
            // the inards and definite, overuning here is a positive thing.
            (r"#", Fit::Yes),
        ];

        for tc in testcases {
            dbg!(&tc.0);
            assert_eq!(possible_fit(tc.0, 1), tc.1);
        }
    }

    #[test]
    fn maybe() {}
}
