use std::mem;

fn main() {
    let input = include_str!("../../input/day9.txt");

    assert_eq!(part1(input), 1938800261);
    assert_eq!(part2(input), 1112);
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|c| c.parse().unwrap())
                .collect()
        })
        .collect()
}

fn part1(input: &str) -> i64 {
    parse(input).iter().map(|h| process_history(h)).sum()
}

fn process_history(data: &[i64]) -> i64 {
    let mut last_sum = 0;
    let mut buf1 = data.to_vec();
    let mut buf2 = vec![];
    let mut stop = false;
    while !stop {
        stop = true;
        last_sum += buf1.last().unwrap();
        for window in buf1.windows(2) {
            let diff = window[1] - window[0];
            buf2.push(diff);
            if diff != 0 {
                stop = false;
            }
        }
        mem::swap(&mut buf1, &mut buf2);
        buf2.clear();
    }

    last_sum
}

fn part2(input: &str) -> i64 {
    parse(input).iter().map(|h| process_history_back(h)).sum()
}

fn process_history_back(data: &[i64]) -> i64 {
    let mut buf1 = data.to_vec();
    let mut buf2 = vec![];
    let mut firsts = vec![];
    let mut stop = false;
    while !stop {
        stop = true;
        firsts.push(buf1[0]);
        for window in buf1.windows(2) {
            let diff = window[1] - window[0];
            buf2.push(diff);
            if diff != 0 {
                stop = false;
            }
        }
        mem::swap(&mut buf1, &mut buf2);
        buf2.clear();
    }

    firsts.iter().rev().fold(0, |acc, x| x - acc)
}
