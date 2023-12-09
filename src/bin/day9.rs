use std::mem;

fn main() {
    let input = include_str!("../../input/day9.txt");

    assert_eq!(part1(input), 1938800261);
    assert_eq!(part2(input), 1112);
}

fn parse(input: &str, rev: bool) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            let iter = line.split_ascii_whitespace().map(|c| c.parse().unwrap());
            if rev {
                iter.rev().collect()
            } else {
                iter.collect()
            }
        })
        .collect()
}

fn part1(input: &str) -> i64 {
    parse(input, false).into_iter().map(process_history).sum()
}

fn part2(input: &str) -> i64 {
    parse(input, true).into_iter().map(process_history).sum()
}

fn process_history(mut data: Vec<i64>) -> i64 {
    let mut last_sum = 0;
    let mut buf = vec![];
    let mut stop = false;
    while !stop {
        stop = true;
        last_sum += data.last().unwrap();
        for window in data.windows(2) {
            let diff = window[1] - window[0];
            buf.push(diff);
            if diff != 0 {
                stop = false;
            }
        }
        mem::swap(&mut data, &mut buf);
        buf.clear();
    }

    last_sum
}
