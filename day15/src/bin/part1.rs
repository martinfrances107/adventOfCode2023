use core::num::Wrapping;

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

fn HashDay15(input: &str) -> u8 {
    let sum = input.bytes().fold(0, |cv, c| {
        let a = Wrapping(cv) + Wrapping(c);
        let b = a * Wrapping(17);
        b.0
    });

    sum
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn hash() {
        let input = r"HASH";
        assert_eq!(HashDay15(input), 52);
    }

    #[test]
    fn simple() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let total: u64 = input.split(',').map(|x| HashDay15(x) as u64).sum();

        assert_eq!(total, 1320);
    }
}
