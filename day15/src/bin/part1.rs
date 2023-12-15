use core::num::Wrapping;

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn part1(input: &str) -> u64 {
    let totals = input.lines().map(single_line).collect::<Vec<_>>();
    totals.iter().sum()
}

fn single_line(input: &str) -> u64 {
    input.split(',').map(|x| hash_day15(x) as u64).sum()
}

fn hash_day15(input: &str) -> u8 {
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
        assert_eq!(hash_day15(input), 52);
    }

    #[test]
    fn simple() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let total: u64 = input.split(',').map(|x| hash_day15(x) as u64).sum();

        assert_eq!(total, 1320);
    }

    #[test]
    fn mulit_line() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let total: u64 = part1(input);

        assert_eq!(total, 2 * 1320);
    }
}
