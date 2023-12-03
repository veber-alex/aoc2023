fn main() {
    let input = include_str!("../../input/day2.txt");

    assert_eq!(part1(input), 3035);
    assert_eq!(part2(input), 66027);
}

#[derive(Debug, Default)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

fn parse(input: &str) -> Vec<Vec<CubeSet>> {
    let mut games = vec![];
    for line in input.lines() {
        let (_, data) = line.split_once(":").unwrap();
        let mut game = vec![];
        for set in data.split(";") {
            let mut cube_set = CubeSet::default();
            for color in set.split(",") {
                let (n, name) = color.trim().split_once(" ").unwrap();
                let n = n.parse().unwrap();
                match name {
                    "red" => cube_set.red = n,
                    "green" => cube_set.green = n,
                    "blue" => cube_set.blue = n,
                    _ => unreachable!(),
                }
            }
            game.push(cube_set);
        }
        games.push(game);
    }

    games
}

fn part1(input: &str) -> u32 {
    parse(input)
        .iter()
        .zip(1..)
        .filter_map(|(game, id)| {
            game.iter()
                .all(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14)
                .then_some(id)
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    parse(input)
        .iter()
        .map(|game| {
            game.iter().map(|set| set.red).max().unwrap()
                * game.iter().map(|set| set.green).max().unwrap()
                * game.iter().map(|set| set.blue).max().unwrap()
        })
        .sum()
}
