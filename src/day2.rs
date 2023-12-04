use std::str::FromStr;

use anyhow::Result;

pub fn part1(input: &str) -> Result<String> {
    let valid_game = Game {
        id: 0,
        green: 13,
        red: 12,
        blue: 14,
    };

    let res: usize = input
        .lines()
        .filter_map(|line| line.parse::<Game>().ok())
        .filter(|game| game.green <= valid_game.green && game.red <= valid_game.red && game.blue <= valid_game.blue)
        .map(|game| game.id)
        .sum();

    Ok(res.to_string())
}

pub fn part2(input: &str) -> Result<String> {
    let res: i32 = input
        .lines()
        .filter_map(|line| line.parse::<Game>().ok())
        .map(|game| game.power())
        .sum();

    Ok(res.to_string())
}

#[derive(Debug)]
struct Game {
    id: usize,
    green: i32,
    red: i32,
    blue: i32,
}

impl Game {
    fn new(id: usize) -> Self {
        Self {
            id,
            green: 0,
            red: 0,
            blue: 0,
        }
    }

    fn power(&self) -> i32 {
        self.green * self.red * self.blue
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let split = s.split_once(": ").ok_or(anyhow::anyhow!("No split found"))?;
        let id = split.0.trim_start_matches("Game ").parse()?;

        let mut sets = split.1.split("; ");
        let game = sets.try_fold(Game::new(id), |mut acc, val| -> Result<Game> {
            let colors: Colors = val.parse()?;
            if colors.green > acc.green {
                acc.green = colors.green;
            }
            if colors.red > acc.red {
                acc.red = colors.red;
            }
            if colors.blue > acc.blue {
                acc.blue = colors.blue;
            }
            Ok(acc)
        })?;

        Ok(game)
    }
}

#[derive(Debug, Default)]
struct Colors {
    green: i32,
    red: i32,
    blue: i32,
}

impl FromStr for Colors {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(", ");
        let mut res = Self::default();
        for color in split {
            match color.split_once(' ') {
                Some((v, "red")) => {
                    res.red = v.parse()?;
                }
                Some((v, "blue")) => {
                    res.blue = v.parse()?;
                }
                Some((v, "green")) => {
                    res.green = v.parse()?;
                }
                _ => panic!("No color found"),
            };
        }

        Ok(res)
    }
}
