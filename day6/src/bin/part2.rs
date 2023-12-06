fn main() {
    println!("{:?}", part2());
}

fn part2() -> usize {
    // Time:        55     99     97     93
    // Distance:   401   1485   2274   1405
    // Time:        55999793
    // Distance:   401148522741405
    let n_ways = nummber_of_ways(&55999793, &401148522741405);
    n_ways
}

fn nummber_of_ways(time: &u128, best_distance: &u128) -> usize {
    let hold_times = 1..*time;
    let distances = hold_times
        .map(|ht| {
            let velocity = ht;
            let running_time = time - ht;
            let dist = velocity * running_time;

            dist
        })
        .filter(|d| d > best_distance)
        .collect::<Vec<_>>();

    distances.len()
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_n_ways2() {
        assert_eq!(nummber_of_ways(&71530, &940200), 71503);
    }
}
