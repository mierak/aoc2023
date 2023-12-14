use std::{collections::HashMap, str::FromStr};

use anyhow::Result;
use itertools::Itertools;

pub fn part1(input: &str) -> Result<usize> {
    let mut grid = input.parse::<Grid>()?;

    grid.roll_north();

    Ok(grid
        .grid
        .iter()
        .filter(|(_, r)| r == &&Rock::Round)
        .map(|((y, _), _)| grid.height - y)
        .sorted()
        .sum())
}
pub fn part2(input: &str) -> Result<usize> {
    let mut grid = input.parse::<Grid>()?;
    let cycles = 1_000_000_000;
    let mut history = HashMap::new();

    let mut cycle_start = 0;
    let mut cycle_len = 0;
    for i in 0..cycles {
        grid.roll_north();
        grid.roll_west();
        grid.roll_south();
        grid.roll_east();
        if let Some(start) = history.insert(grid.to_key(), i + 1) {
            cycle_start = start;
            cycle_len = i - cycle_start + 1;
            break;
        }
    }

    let left_to_process = (cycles - cycle_start) % cycle_len;
    for _ in 0..left_to_process {
        grid.roll_north();
        grid.roll_west();
        grid.roll_south();
        grid.roll_east();
    }

    Ok(grid
        .grid
        .iter()
        .filter(|(_, r)| r == &&Rock::Round)
        .map(|((y, _), _)| grid.height - y)
        .sorted()
        .sum())
}

#[derive(PartialEq, Eq, Clone)]
enum Rock {
    Round,
    Cube,
}
#[derive(Clone, PartialEq, Eq)]
struct Grid {
    grid: HashMap<(usize, usize), Rock>,
    width: usize,
    height: usize,
}

impl Grid {
    fn to_key(&self) -> Vec<(usize, usize)> {
        self.grid.keys().copied().collect_vec()
    }

    fn roll_north(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let key = match self.grid.get(&(y, x)) {
                    Some(val) if val == &Rock::Cube => continue,
                    None => continue,
                    _ => (y, x),
                };
                let mut y = key.0;

                loop {
                    if y == 0 {
                        break;
                    }
                    if self.grid.contains_key(&(y - 1, key.1)) {
                        break;
                    }
                    y -= 1;
                }
                self.grid.remove(&key);
                self.grid.insert((y, key.1), Rock::Round);
            }
        }
    }

    fn roll_south(&mut self) {
        let keys = self.grid.keys().copied().sorted_by(|a, b| b.0.cmp(&a.0)).collect_vec();
        for key in keys {
            if self.grid.get(&key).is_some_and(|r| r == &Rock::Cube) {
                continue;
            }
            let mut y = key.0;

            loop {
                if y == self.height - 1 {
                    break;
                }
                if self.grid.contains_key(&(y + 1, key.1)) {
                    break;
                }
                y += 1;
            }
            self.grid.remove(&key);
            self.grid.insert((y, key.1), Rock::Round);
        }
    }

    fn roll_west(&mut self) {
        let keys = self.grid.keys().copied().sorted_by(|a, b| a.1.cmp(&b.1)).collect_vec();
        for key in keys {
            if self.grid.get(&key).is_some_and(|r| r == &Rock::Cube) {
                continue;
            }
            let mut x = key.1;

            loop {
                if x == 0 {
                    break;
                }
                if self.grid.contains_key(&(key.0, x - 1)) {
                    break;
                }
                x -= 1;
            }
            self.grid.remove(&key);
            self.grid.insert((key.0, x), Rock::Round);
        }
    }

    fn roll_east(&mut self) {
        let keys = self.grid.keys().copied().sorted_by(|a, b| b.1.cmp(&a.1)).collect_vec();
        for key in keys {
            if self.grid.get(&key).is_some_and(|r| r == &Rock::Cube) {
                continue;
            }
            let mut x = key.1;

            loop {
                if x == self.width - 1 {
                    break;
                }
                if self.grid.contains_key(&(key.0, x + 1)) {
                    break;
                }
                x += 1;
            }
            self.grid.remove(&key);
            self.grid.insert((key.0, x), Rock::Round);
        }
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let grid: HashMap<_, _> = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| match c {
                    'O' => Some(((y, x), Rock::Round)),
                    '#' => Some(((y, x), Rock::Cube)),
                    _ => None,
                })
            })
            .collect();
        Ok(Grid {
            width: grid.keys().map(|(_, x)| x).max().unwrap_or(&0) + 1,
            height: grid.keys().map(|(y, _)| y).max().unwrap_or(&0) + 1,
            grid,
        })
    }
}
