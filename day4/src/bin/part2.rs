fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part2(input));
}

fn collect_winings<'a>(
    reference_list: &Vec<&'a str>,
    active_list: &Vec<&'a str>,
) -> (Vec<&'a str>, Vec<&'a str>) {
    // A card form the original list that contains winnings numbers.
    let mut winners = vec![];
    // Copies A colleciton of won cards.
    let mut copies = vec![];
    for line in active_list.iter() {
        if let Some((header, w_p)) = line.split_once(':') {
            // Recover game number from header.
            let card_num = if let Some(card_num_str) = header.strip_prefix(&"Card ") {
                if let Ok(card_num) = card_num_str.parse::<usize>() {
                    card_num
                } else {
                    dbg!("Could not parse game_id as us32");
                    panic!();
                }
            } else {
                dbg!("Malformed Card entry: Game does not start with 'Card '");
                panic!();
            };
            let card_index = card_num - 1;
            if let Some((w, p)) = w_p.split_once('|') {
                let w_numbers = w
                    .split(' ')
                    .filter_map(|s| s.parse::<u32>().ok())
                    .collect::<Vec<u32>>();
                let p_numbers = p
                    .split(' ')
                    .filter_map(|s| s.parse::<u32>().ok())
                    .collect::<Vec<u32>>();

                let mut n_matches = 0;
                for w in &w_numbers {
                    for p in &p_numbers {
                        if *w == *p {
                            n_matches += 1;
                        }
                    }
                }

                if n_matches != 0 {
                    // current card is a wining card, collect copies.
                    for copy_card_index in card_index + 1..card_num + n_matches {
                        // Want index here
                        copies.push(reference_list[copy_card_index - 1]);
                    }
                    winners.push(line.clone());
                }
                dbg!(&(&winners, &copies));
            }
        }
    }
    let out = (winners, copies);

    out
}
fn part2(input: &str) -> usize {
    let original_cards: Vec<&str> = input.lines().collect();

    let (winners, copies1) = collect_winings(&original_cards, &original_cards);
    println!("Round 2");
    let (_, copies2) = collect_winings(&original_cards, &copies1);

    winners.len() + copies1.len() + copies2.len()
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn example() {
        let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part2(input), 30usize)
    }
}
