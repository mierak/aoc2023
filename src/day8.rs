use std::{collections::HashMap, str::FromStr};

use anyhow::{bail, Context, Result};
use itertools::Itertools;

pub fn part1(input: &str) -> Result<i32> {
    let game: Game = input.parse()?;
    let finish = "ZZZ";
    let mut current = "AAA";
    let mut i = 0;
    let mut count = 0;

    while current != finish {
        match game.moves[i] {
            Move::Left => {
                current = &game.instructions.get(current).context("No mapping found")?.0;
            }
            Move::Right => {
                current = &game.instructions.get(current).context("No mapping found")?.1;
            }
        }

        i = (i + 1) % game.moves.len();
        count += 1;
    }

    Ok(count)
}

pub fn part2(input: &str) -> Result<i64> {
    let game: Game = input.parse()?;
    let mut current = game.instructions.keys().filter(|k| k.ends_with('A')).collect_vec();
    let mut counts = (0..current.len()).map(|_| 0).collect_vec();

    for node_idx in 0..current.len() {
        let mut i = 0;
        while !current[node_idx].ends_with('Z') {
            match game.moves[i] {
                Move::Left => {
                    current[node_idx] = &game.instructions.get(current[node_idx]).context("No mapping found")?.0;
                }
                Move::Right => {
                    current[node_idx] = &game.instructions.get(current[node_idx]).context("No mapping found")?.1;
                }
            }

            i = (i + 1) % game.moves.len();
            counts[node_idx] += 1;
        }
    }

    Ok(counts.into_iter().fold(1, lcm))
}

#[derive(Debug)]
enum Move {
    Left,
    Right,
}

#[derive(Debug)]
struct Game {
    moves: Vec<Move>,
    instructions: HashMap<String, (String, String)>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();
        let moves = lines
            .next()
            .context("No moves")?
            .chars()
            .map(|c| match c {
                'L' => Ok(Move::Left),
                'R' => Ok(Move::Right),
                _ => bail!("Invalid move"),
            })
            .try_collect()?;
        lines.next().context("No empty line")?;

        let instructions = lines
            .map(|line| -> Result<_> {
                let (key, mapping) = line.split_once(" = ").context("Invalid mapping key")?;
                let (from, to) = mapping
                    .trim_start_matches('(')
                    .trim_end_matches(')')
                    .split_once(", ")
                    .context("Invalid mapping")?;

                Ok((key.to_string(), (from.to_string(), to.to_string())))
            })
            .try_collect()?;
        Ok(Self { moves, instructions })
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}
