fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn part1(input: &str) -> u32 {
    let mut lines = input.lines();

    let time_line = lines.next().expect("time line must exits");
    let dist_line = lines.next().expect("distance line must exits");

    dbg!(time_line);
    let Some((_header, time_block)) = time_line.split_once(':') else {
        println!("Failed to extract time ");
        return 0;
    };
    dbg!(&time_block.trim());
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

    for (time, distance) in inputs {
        //
        println!("time {time} distance {distance}");
    }
    1u32
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn example() {
        let input = r"Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part1(input), 142u32)
    }
}
