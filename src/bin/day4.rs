fn main() {
    let input = include_str!("../../input/day4.txt");

    assert_eq!(part1(input), 22193);
    assert_eq!(part2(input), 5625994);
}

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
    copies: u32,
}

fn parse(input: &str) -> Vec<Card> {
    let mut cards = vec![];
    for line in input.lines() {
        let (_, data) = line.split_once(':').unwrap();
        let (win, nums) = data.split_once('|').unwrap();
        let winning_numbers = win
            .trim()
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let numbers = nums
            .trim()
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        cards.push(Card {
            winning_numbers,
            numbers,
            copies: 1,
        })
    }

    cards
}

fn part1(input: &str) -> u32 {
    let cards = parse(input);
    let mut sum = 0;
    let mut score = 0;
    for card in cards {
        for num in card.numbers {
            if card.winning_numbers.contains(&num) {
                if score == 0 {
                    score = 1
                } else {
                    score *= 2;
                }
            }
        }
        sum += score;
        score = 0
    }

    sum
}

fn part2(input: &str) -> u32 {
    let mut cards = parse(input);
    for idx in 0..cards.len() {
        let card = &mut cards[idx];
        let score = card
            .winning_numbers
            .iter()
            .filter(|win| card.numbers.contains(win))
            .count();
        let card_copies = card.copies;
        for next in 1..=score {
            cards[idx + next].copies += card_copies
        }
    }

    cards.iter().map(|card| card.copies).sum()
}
