fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part2(input));
}

fn collect_winings<'a>(reference_list: &Vec<&'a str>, active_list: &Vec<&'a str>) -> Vec<&'a str> {
    // Copies A colleciton of won cards.
    let mut copies = vec![];
    for line in active_list.iter() {
        if let Some((header, w_p)) = line.split_once(':') {
            // Recover game number from header.
            let card_num = if let Some(card_num_str) = header.strip_prefix("Card ") {
                if let Ok(card_num) = card_num_str.trim().parse::<usize>() {
                    // dbg!(&card_num);
                    card_num
                } else {
                    dbg!(&line);
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
                    // dbg!(n_matches);
                    for copy_card_index in card_index + 1..card_index + 1 + n_matches {
                        if let Some(won_card) = reference_list.get(copy_card_index) {
                            // dbg!("pushing card");
                            // dbg!(won_card);
                            copies.push(*won_card);
                        }
                    }
                }
                // dbg!(&(&winners, &copies));
            }
        }
    }

    copies
}
fn part2(input: &str) -> usize {
    let original_cards: Vec<&str> = input.lines().collect();
    let mut active_list = original_cards.clone();
    let mut total = 0;

    let mut lc = 0;
    loop {
        let winnings = collect_winings(&original_cards, &active_list);
        total += active_list.len();
        // dbg!(&winnings);
        // dbg!(total);
        if active_list.is_empty() {
            break;
        }
        if lc > 2000000 {
            assert!(false);
        }
        lc += 1;
        active_list = winnings.clone();
    }
    total
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

    #[test]
    // test data on the first round ... just considering the winnings of card1
    // should generate next 4 ... 2,3,4,5
    fn winnings_from_one_iteration() {
        let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let input = input.lines().collect::<Vec<&str>>();
        let single = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let single = single.lines().collect::<Vec<&str>>();
        let copies = collect_winings(&input, &single);
        println!("{:#?}", &copies);
        assert_eq!(
            copies,
            vec![
                r"Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
                r"Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
                r"Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
                r"Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"
            ]
        )
    }

    #[test]
    fn winnings_from_three_cards() {
        let input_str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let input = input_str.lines().collect::<Vec<&str>>();

        let copies1 = collect_winings(&input, &input);

        assert_eq!(
            copies1,
            vec![
                r"Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
                r"Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
                r"Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            ]
        );

        // Treat the won copies as "active" and process them.
        let copies2 = collect_winings(&input, &copies1);
        assert_eq!(
            copies2,
            vec![r"Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",]
        );

        let copies3 = collect_winings(&input, &copies2);
        assert_eq!(copies3.len(), 0);
        let count = part2(&input_str);
        assert_eq!(count, 7);
    }

    #[test]
    fn winnings_from_four_cards() {
        // the story of card 4 ( 1 original + 7 copies)
        let input_str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
        let input = input_str.lines().collect::<Vec<&str>>();

        let copies1 = collect_winings(&input, &input);

        // 3 copies of "card 4" here.
        assert_eq!(
            copies1,
            vec![
                // won from processing card 1
                r"Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
                r"Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
                r"Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
                // won from processing card 2
                r"Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
                r"Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
                // won from processing card 3
                r"Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            ]
        );

        // Treat the won copies as "active" and process them.
        let copies2 = collect_winings(&input, &copies1);
        // 3 copies of "card 4" here.
        assert_eq!(
            copies2,
            vec![
                // won from processing card 2
                r"Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
                r"Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
                // won from processing card 3
                r"Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
                // won from processing card 3
                r"Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            ]
        );
        let copies3 = collect_winings(&input, &copies2);
        // 3 copies of "card 4" here. == makes 7 copies in total
        assert_eq!(
            copies3,
            vec![
                // won from processing card 3
                r"Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            ]
        );
    }

    #[test]
    fn card6() {
        let input_str = r"Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let input = input_str.lines().collect::<Vec<&str>>();

        let copies1 = collect_winings(&input, &input);

        assert_eq!(copies1, Vec::<&str>::new());

        // Treat the won copies as "active" and process them.
        let copies2 = collect_winings(&input, &copies1);
        assert_eq!(copies2, Vec::<&str>::new());

        let count = part2(&input_str);
        assert_eq!(count, 1);
    }
}
