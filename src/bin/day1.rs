fn main() {
    let input = include_str!("../../input/day1.txt");

    assert_eq!(part1(input), 54630);
    assert_eq!(part2(input), 54770);
}

fn part1(input: &str) -> u32 {
    let mut sum: u32 = 0;
    let find = |b: &u8| matches!(b, b'0'..=b'9');
    for line in input.lines() {
        let first = line.bytes().find(find).unwrap() as u32;
        let last = line.bytes().rev().find(find).unwrap() as u32;
        sum += (first - 48) * 10 + (last - 48)
    }

    sum
}

fn part2(input: &str) -> u32 {
    input.lines().map(scan_line).sum()
}

fn scan_line(mut line: &str) -> u32 {
    let first = loop {
        break if line.starts_with('1') || line.starts_with("one") {
            1
        } else if line.starts_with('2') || line.starts_with("two") {
            2
        } else if line.starts_with('3') || line.starts_with("three") {
            3
        } else if line.starts_with('4') || line.starts_with("four") {
            4
        } else if line.starts_with('5') || line.starts_with("five") {
            5
        } else if line.starts_with('6') || line.starts_with("six") {
            6
        } else if line.starts_with('7') || line.starts_with("seven") {
            7
        } else if line.starts_with('8') || line.starts_with("eight") {
            8
        } else if line.starts_with('9') || line.starts_with("nine") {
            9
        } else {
            line = &line[1..];
            continue;
        };
    };

    let last = loop {
        break if line.ends_with('1') || line.ends_with("one") {
            1
        } else if line.ends_with('2') || line.ends_with("two") {
            2
        } else if line.ends_with('3') || line.ends_with("three") {
            3
        } else if line.ends_with('4') || line.ends_with("four") {
            4
        } else if line.ends_with('5') || line.ends_with("five") {
            5
        } else if line.ends_with('6') || line.ends_with("six") {
            6
        } else if line.ends_with('7') || line.ends_with("seven") {
            7
        } else if line.ends_with('8') || line.ends_with("eight") {
            8
        } else if line.ends_with('9') || line.ends_with("nine") {
            9
        } else {
            line = &line[..line.len() - 1];
            continue;
        };
    };

    first * 10 + last
}
