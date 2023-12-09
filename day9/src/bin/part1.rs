fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn compute_diff<I>(values: I) -> Vec<i64>
where
    I: IntoIterator<Item = i64>,
{
    let mut iter = values.into_iter();
    let mut prev: i64 = iter.next().unwrap();
    iter.map(|x| {
        let diff: i64 = x - prev;
        prev = x;
        diff
    })
    .collect::<Vec<i64>>()
}

fn extrapolate(values: &mut Vec<Vec<i64>>) -> i64 {
    //The row below is all zeros
    // so the initial increament is zero.
    let mut inc = 0;
    values.iter_mut().rev().for_each(|row| {
        let last_value = row.last().unwrap();
        let next_value = last_value + inc;
        row.push(next_value);
        inc = next_value;
    });

    let new_value = *values[0].last().unwrap();

    new_value
}
fn part1(input: &str) -> i64 {
    let new_values = input
        .lines()
        .map(|line| {
            let mut lc = 0;
            let elements: Vec<&str> = line.split(' ').collect();
            // dbg!(&elements);
            let mut readings: Vec<i64> = elements
                .iter()
                .map(|element| element.parse::<i64>().ok().unwrap())
                .collect();

            let mut rows = Vec::from(vec![readings.clone()]);
            loop {
                let diff = compute_diff(readings);
                if diff.iter().all(|x| *x == 0) {
                    break;
                }
                rows.push(diff.clone());

                if lc > 10_000_000 {
                    panic!();
                }

                // dbg!(&rows);
                readings = diff;
                lc += 1;
            }
            rows
        })
        .map(|mut rows| extrapolate(&mut rows))
        .collect::<Vec<i64>>();

    new_values.iter().sum()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn diff() {
        let a: Vec<i64> = vec![1, 2, 3, 4, 5];
        // let a = a.iter();
        let diff = compute_diff(a);

        assert_eq!(diff, [1, 1, 1, 1]);
    }

    #[test]
    fn example() {
        let input = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(part1(input), 114)
    }
}
