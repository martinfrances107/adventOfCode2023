extern crate nom;

fn main() {
    let input = include_str!("./input1.txt");
    println!("{}", part1(input));
}

fn part1(input: &str) -> String {
    let recoverer_ns_as_string = input
        .lines()
        .map(|line| {
            let number_str = line.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>();
            if let Some(first) = number_str.first() {
                let recoverd_number = match number_str.last() {
                    Some(last) => {
                        let mut number = first.to_string();
                        number.push(*last);
                        number
                    }
                    None => {
                        // for single digit .first() and .last() are identical
                        // so this point is never reached.
                        panic!("if there is a first, then there is a last");
                    }
                };
                recoverd_number
            } else {
                String::from("0")
            }
        })
        .collect::<Vec<_>>();

    let mut total = 0u128;
    for n_str in recoverer_ns_as_string {
        if let Ok(n) = n_str.parse::<u128>() {
            total += n;
        }
    }
    total.to_string()
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
        assert_eq!(part1(input), "142")
    }
}
