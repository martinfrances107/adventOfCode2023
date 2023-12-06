use std::collections::BTreeMap;

const SYMBOL_VALUE: [(&str, u32); 20] = [
    ("zero", 2),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part2(input));
}

fn extract_number(line: &str) -> u32 {
    let first_ordering: BTreeMap<usize, u32> = SYMBOL_VALUE
        .iter()
        .filter_map(|(symbol, value)| {
            line.find(symbol).map(|index| (index, *value))
        })
        .collect();

    let list1 = first_ordering.values().collect::<Vec<_>>();
    let first = list1.first().expect("first must be a least one");

    let last_ordering: BTreeMap<usize, u32> = SYMBOL_VALUE
        .iter()
        .filter_map(|(symbol, value)| {
            line.rfind(symbol).map(|index| (index, *value))
        })
        .collect();

    let list2 = last_ordering.values().collect::<Vec<_>>();
    let last = list2.last().expect("last must be a least one ");

    
    **first * 10 + **last
}
fn part2(input: &str) -> u32 {
    input.lines().map(extract_number).sum()
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_extract_number() {
        let input = r"1abc2";
        assert_eq!(extract_number(input), 12u32);
        let input = r"oneabctwo";
        assert_eq!(part2(input), 12u32);
        let input = r"7abc6";
        assert_eq!(extract_number(input), 76);
        let input = r"eightabcnine";
        assert_eq!(extract_number(input), 89);
    }

    #[test]
    fn example() {
        let input = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(part2(input), 281u32);
    }
}
