fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let number_str = line.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>();
            let mut n = 0;
            if let Some(first) = number_str.first() {
                if let Some(tens) = first.to_digit(10) {
                    n += tens * 10;
                }
            }
            if let Some(first) = number_str.last() {
                if let Some(units) = first.to_digit(10) {
                    n += units;
                }
            }
            n
        })
        .sum()
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn example() {
        let input = r"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!(part1(input), 142u32)
    }
}
