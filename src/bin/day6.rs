use std::iter::zip;

fn main() {
    let input = include_str!("../../input/day6.txt");

    assert_eq!(part1(input), 74698);
    assert_eq!(part2(input), 27563421);
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance_record: u64,
}

fn part1(input: &str) -> u64 {
    let (time, distance) = input.split_once('\n').unwrap();
    let times = time.split_once(':').unwrap().1.split_ascii_whitespace();
    let distances = distance.split_once(':').unwrap().1.split_ascii_whitespace();
    let races: Vec<Race> = zip(times, distances)
        .map(|(t, z)| Race {
            time: t.parse().unwrap(),
            distance_record: z.parse().unwrap(),
        })
        .collect();

    solve(&races)
}

fn part2(input: &str) -> u64 {
    let (time, distance) = input.split_once('\n').unwrap();
    let times: String = time
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .collect();
    let distances: String = distance
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .collect();

    let race = Race {
        time: times.parse().unwrap(),
        distance_record: distances.parse().unwrap(),
    };

    solve(&[race])
}

fn solve(races: &[Race]) -> u64 {
    let mut res = 1;
    for race in races {
        let mut win_ways = 0;
        let mut win_start = 0;
        for wait in 1..race.time {
            if wait * (race.time - wait) > race.distance_record {
                win_start = wait;
                break;
            }
        }
        for wait in (1..race.time).rev() {
            if wait * (race.time - wait) > race.distance_record {
                win_ways = wait - win_start + 1;
                break;
            }
        }
        res *= win_ways;
    }

    res
}
