use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn part1(input: &str) -> u32 {
    1u32
}
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Hash, Eq)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    Ace,
}

impl From<&char> for Card {
    fn from(x: &char) -> Self {
        match x {
            'A' => Card::Ace,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => {
                panic!("malformed card")
            }
        }
    }
}
#[derive(Debug, PartialOrd, PartialEq, Eq, Ord)]
enum Hand {
    // High card -- remaining 4 singles
    HighCard(Card, Card, Card, Card, Card),
    // kk 4,3, 2 -- pair remaing three singles
    OnePair(Card, Card, Card, Card),
    // kk qq 2 -- pair, pair, highcard
    TwoPair(Card, Card, Card),
    // kkk q j -- group of three,  high card, low card
    ThreeOfAKind(Card, Card, Card),
    // kkk qq j -- triple, pair, high card
    FullHouse(Card, Card),
    // kkkk two -- quad, high card
    FourOfAKind(Card, Card),
    /// From a shoe five aces are possible
    FiveOfAKind(Card),
}

impl From<&str> for Hand {
    fn from(line: &str) -> Hand {
        let mut hand = [Card::Two; 5];
        for (i, c) in line.chars().take(5).enumerate() {
            hand[i] = (&c).into();
        }

        let unique_cards: BTreeSet<Card> = hand.map(|c| c).into();
        // dbg!(&unique_cards);

        let mut histogram = BTreeMap::new();
        for uc in unique_cards {
            for h in hand.iter() {
                if uc == *h {
                    // increment tally.
                    match histogram.get_mut(&uc) {
                        Some(count) => {
                            *count = *count + 1;
                        }
                        None => {
                            histogram.insert(uc, 1);
                        }
                    }
                }
            }
        }

        // dbg!(&histogram);

        let heap: BinaryHeap<_> = histogram
            .iter()
            .map(|(card, count)| {
                // Reverse order so when sorted
                // it produces a list sorted by group.
                (count, card)
            })
            .collect();

        let mut list = heap.into_sorted_vec();
        list.reverse();

        // dbg!(&list);
        // Extract from the histogram the most and second most common card.

        let mut iter = list.iter();
        let (r1, c1) = iter
            .next()
            .expect("There must always be a primary grouping");
        let r2_c2 = iter.next();

        let p = match (r1, r2_c2) {
            (5, None) => Hand::FiveOfAKind(**c1),
            (4, Some((_, c2))) => Hand::FourOfAKind(**c1, **c2),
            (3, Some((&2, c2))) => Hand::FullHouse(**c1, **c2),
            (3, Some((1, c2))) => {
                // Three of a kind, plus high card, plus low cards
                let (_, low_card) = iter.next().expect("must have low card");
                Hand::ThreeOfAKind(**c1, **c2, **low_card)
            }
            (2, Some((2, c2))) => {
                // two pair , plus high card
                let (_, low_card) = iter.next().expect("must have low card");
                if **c1 > **c2 {
                    Hand::TwoPair(**c1, **c2, **low_card)
                } else {
                    Hand::TwoPair(**c2, **c1, **low_card)
                }
            }
            (2, Some((1, spare0))) => {
                let (_, spare1) = iter.next().expect("must have low card");
                let (_, spare2) = iter.next().expect("must have low card");
                let mut spares = [spare0, spare1, spare2];
                spares.sort();
                Hand::OnePair(**c1, **spares[2], **spares[1], **spares[0])
            }
            // Default to lowest hand possible
            (1, Some((1, spare0))) => {
                let mut spare = vec![spare0];
                let mut other_spares = iter.map(|(_count, card)| card).collect::<Vec<&&Card>>();
                spare.append(&mut other_spares);
                spare.sort();
                Hand::HighCard(**c1, **spare[3], **spare[2], **spare[1], **spare[0])
            }

            _ => {
                panic!("bad decode");
            }
        };

        // dbg!(&hand);
        p
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_calc_hands() {
        const TESTSET: [(Hand, &str); 7] = [
            (Hand::TwoPair(Card::Three, Card::Two, Card::Four), "23432"),
            (
                Hand::OnePair(Card::Ace, Card::Four, Card::Three, Card::Two),
                "A23A4",
            ),
            (Hand::FourOfAKind(Card::Ace, Card::Eight), "AA8AA"),
            (Hand::FullHouse(Card::Three, Card::Two), "23332"),
            (
                Hand::ThreeOfAKind(Card::T, Card::Nine, Card::Eight),
                "TTT98",
            ),
            (
                Hand::HighCard(Card::Six, Card::Five, Card::Four, Card::Three, Card::Two),
                "23456",
            ),
            (Hand::FiveOfAKind(Card::Ace), "AAAAA"),
        ];

        for (h, line) in TESTSET {
            assert_eq!(h, line.into())
        }
    }

    #[test]
    fn calc_rank() {
        let input = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 48";

        let mut hands = input
            .lines()
            .map(|line| {
                let (hand_str, bid_str) = line.split_once(' ').expect("one wite space");
                let hand: Hand = hand_str.into();
                let _bid = bid_str.parse::<u32>().expect("failed to parse bid");
                hand
            })
            .collect::<Vec<Hand>>();
        // Ranked hands are sorted hands.

        dbg!(&hands);
        hands.sort();

        let expected_ranked_hanks = vec![
            Hand::OnePair(Card::Three, Card::K, Card::T, Card::Two),
            Hand::TwoPair(Card::J, Card::T, Card::K),
            Hand::TwoPair(Card::K, Card::Seven, Card::Six),
            Hand::ThreeOfAKind(Card::Five, Card::J, Card::T),
            Hand::ThreeOfAKind(Card::Q, Card::Ace, Card::J),
        ];

        assert_eq!(expected_ranked_hanks, hands);
    }
}
