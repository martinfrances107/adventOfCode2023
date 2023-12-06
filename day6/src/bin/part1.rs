fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn part1(input: &str) -> u32 {
    let mut lines = input.lines();

    let time_line = lines.next().expect("time line must exits");
    let dist_line = lines.next().expect("distance line must exits");

    // dbg!(time_line);
    let Some((_header, time_block)) = time_line.split_once(':') else {
        println!("Failed to extract time ");
        return 0;
    };
    // dbg!(&time_block.trim());
    let times = time_block
        .trim()
        .split(' ')
        .filter_map(|x| x.trim().parse::<u32>().ok())
        .collect::<Vec<u32>>();

    let Some((_header, distance_block)) = dist_line.split_once(':') else {
        println!("Failed to extract distance");
        return 0;
    };
    let distances = distance_block
        .split(' ')
        .filter_map(|x| x.trim().parse::<u32>().ok())
        .collect::<Vec<_>>();

    let inputs = times.iter().zip(distances.iter());

    let mut n_ways = vec![];
    for (time, distance) in inputs {
        //
        // println!("time {time} distance {distance}");
        n_ways.push(nummber_of_ways(time, distance));
    }

    let mut product = 1;
    for n in n_ways {
        product *= n;
    }
    product
}

// fn compute_distance(ht: u32, time: u32) -> u32 {
//     let velocity = ht;
//     let running_time = time - ht;
//     let dist = velocity * running_time;

//     dist
// }

fn nummber_of_ways(time: &u32, best_distance: &u32) -> u32 {
    // dbg!(&best_distance);
    let hold_times = 1..*time;
    let distances: Vec<u32> = hold_times
        .map(|ht| {
            let velocity = ht;
            let running_time = time - ht;
            let dist = velocity * running_time;

            dist
        })
        .filter(|d| d > best_distance)
        .collect::<Vec<_>>();

    // dbg!(&distances);
    distances.len() as u32
}

#[cfg(test)]
mod test {

    #[test]
    fn test_n_ways() {
        // dataset: (time, distance, n_ways)
        let dataset = [[7, 9, 4], [15, 40, 8], [30, 200, 9]];

        for [time, best_distance, n_ways] in dataset {
            assert_eq!(nummber_of_ways(&time, &best_distance), n_ways);
        }
    }
    use super::*;
    #[test]
    fn example() {
        let input = r"Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part1(input), 288u32)
    }
}
