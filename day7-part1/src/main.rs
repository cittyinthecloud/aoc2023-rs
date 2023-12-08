#![feature(test)]
extern crate test;
use std::{fs, iter::zip};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Card {
    card_rank: usize,
}

impl Card {
    fn from_char(card_char: &char) -> Card {
        Card {
            card_rank: match card_char {
                'A' => 13,
                'K' => 12,
                'Q' => 11,
                'J' => 10,
                'T' => 9,
                '9' => 8,
                '8' => 7,
                '7' => 6,
                '6' => 5,
                '5' => 4,
                '4' => 3,
                '3' => 2,
                '2' => 1,
                '1' => 0,
                _ => unreachable!(),
            },
        }
    }
}

impl Default for Card {
    fn default() -> Self {
        Self {
            card_rank: Default::default(),
        }
    }
}

#[derive(Debug, Ord)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
    bet: u16,
}

impl Hand {
    fn new(hand_str: &str, bet: u16) -> Self {
        let mut cards = [Default::default(); 5];
        let mut counts: [i32; 14] = [0; 14];
        let mut max_count = 0;
        let mut pair_count = 0;

        let mut found_three_of_a_kind = false;

        let mut found_hand_type = None;

        for (i, rank_char) in hand_str.char_indices() {
            let card: Card = Card::from_char(&rank_char);
            cards[i] = card;
            counts[card.card_rank] += 1;
            if counts[card.card_rank] > max_count {
                max_count = counts[card.card_rank];
            }

            if counts[card.card_rank] == 5 {
                found_hand_type = Some(HandType::FiveOfAKind);
            } else if counts[card.card_rank] == 4 {
                found_hand_type = Some(HandType::FourOfAKind);
            } else if counts[card.card_rank] == 3 {
                pair_count -= 1;
                found_three_of_a_kind = true;
            } else if counts[card.card_rank] == 2 {
                pair_count += 1;
            }
        }

        let hand_type = if let Some(found_hand_type) = found_hand_type {
            found_hand_type
        } else if found_three_of_a_kind {
            if pair_count == 1 {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        } else if pair_count == 2 {
            HandType::TwoPair
        } else if pair_count == 1 {
            HandType::OnePair
        } else {
            HandType::HighCard
        };

        Self {
            cards,
            hand_type,
            bet,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }

        for (our_card, other_card) in zip(self.cards, other.cards) {
            match our_card.partial_cmp(&other_card) {
                Some(core::cmp::Ordering::Equal) => {}
                ord => return ord,
            }
        }

        self.bet.partial_cmp(&other.bet)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.hand_type == other.hand_type
    }
}

impl Eq for Hand {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let answer = do_aoc(&input);

    println!("{answer}")
}

fn do_aoc(input: &str) -> u64 {
    let mut hands: Vec<Hand> = input
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(hand_str, bet)| Hand::new(hand_str, bet.parse().unwrap()))
        .collect();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| ((i + 1) as u64) * (hand.bet as u64))
        .sum()
}

#[bench]
fn bench_day7(b: &mut test::Bencher) {
    let file = fs::read_to_string("input").unwrap();

    b.iter(|| test::black_box(do_aoc(&file)));
}
