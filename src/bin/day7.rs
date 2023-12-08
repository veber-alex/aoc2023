use std::iter::zip;

fn main() {
    let input = include_str!("../../input/day7.txt");

    assert_eq!(part1(input), 256448566);
    assert_eq!(part2(input), 254412181);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

use HandKind::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    kind: HandKind,
    cards: [u8; 5],
    bid: u32,
}

fn solve(mut hands: Vec<Hand>) -> u32 {
    hands.sort_unstable();

    hands
        .iter()
        .zip(1..)
        .map(|(hand, rank)| rank * hand.bid)
        .sum()
}

fn part1(input: &str) -> u32 {
    let hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let (symbols, bid) = line.split_once(' ').unwrap();
            let bid = bid.parse().unwrap();
            let mut cards = [0; 5];
            let mut counter = [0; 13];
            for (sym, card) in zip(symbols.as_bytes(), &mut cards) {
                *card = match sym {
                    b'2'..=b'9' => sym - 50,
                    b'T' => 8,
                    b'J' => 9,
                    b'Q' => 10,
                    b'K' => 11,
                    b'A' => 12,
                    _ => unreachable!(),
                };
                counter[*card as usize] += 1;
            }

            let mut kind = HighCard;
            for count in counter {
                kind = match (count, kind) {
                    (5, _) => FiveOfAKind,
                    (4, _) => FourOfAKind,
                    (3, HighCard) => ThreeOfAKind,
                    (3, OnePair) => FullHouse,
                    (2, HighCard) => OnePair,
                    (2, OnePair) => TwoPair,
                    (2, ThreeOfAKind) => FullHouse,
                    (0 | 1, _) => continue,
                    _ => unreachable!(),
                };
            }

            Hand { cards, kind, bid }
        })
        .collect();

    solve(hands)
}

fn part2(input: &str) -> u32 {
    let hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let (symbols, bid) = line.split_once(' ').unwrap();
            let bid = bid.parse().unwrap();
            let mut cards = [0; 5];
            let mut counter = [0; 13];
            for (sym, card) in zip(symbols.as_bytes(), &mut cards) {
                *card = match sym {
                    b'J' => 0,
                    b'2'..=b'9' => sym - 49,
                    b'T' => 9,
                    b'Q' => 10,
                    b'K' => 11,
                    b'A' => 12,
                    _ => unreachable!(),
                };
                counter[*card as usize] += 1;
            }

            let mut kind = HighCard;
            for count in &counter[1..] {
                kind = match (count, kind) {
                    (5, _) => FiveOfAKind,
                    (4, _) => FourOfAKind,
                    (3, HighCard) => ThreeOfAKind,
                    (3, OnePair) => FullHouse,
                    (2, HighCard) => OnePair,
                    (2, OnePair) => TwoPair,
                    (2, ThreeOfAKind) => FullHouse,
                    (0 | 1, _) => continue,
                    _ => unreachable!(),
                };
            }

            let jokers = counter[0];
            kind = match (jokers, kind) {
                (0, _) => kind,
                (1, FourOfAKind) => FiveOfAKind,
                (1, ThreeOfAKind) => FourOfAKind,
                (1, TwoPair) => FullHouse,
                (1, OnePair) => ThreeOfAKind,
                (1, HighCard) => OnePair,
                (2, ThreeOfAKind) => FiveOfAKind,
                (2, OnePair) => FourOfAKind,
                (2, HighCard) => ThreeOfAKind,
                (3, OnePair) => FiveOfAKind,
                (3, HighCard) => FourOfAKind,
                (4 | 5, _) => FiveOfAKind,
                _ => unreachable!(),
            };

            Hand { cards, kind, bid }
        })
        .collect();

    solve(hands)
}
