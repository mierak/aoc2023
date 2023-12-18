use anyhow::{bail, Result};
use itertools::Itertools;
use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

pub fn part1(input: &str) -> Result<i64> {
    let plan = input.parse::<Plan>()?;
    let mut current = Coord { x: 0, y: 0 };
    let mut lagoon = Vec::new();
    let mut result = HashMap::new();
    for dig in plan.0 {
        current = dig.commit(current, &mut lagoon);
    }

    let max_x = lagoon.iter().map(|e| e.start.x).max().unwrap();
    let max_y = lagoon.iter().map(|e| e.start.y).max().unwrap();
    let min_x = lagoon.iter().map(|e| e.start.x).min().unwrap();
    let min_y = lagoon.iter().map(|e| e.start.y).min().unwrap();

    for y in min_y..=max_y {
        let mut is_inside = false;
        for x in min_x..=max_x {
            lagoon
                .iter()
                .sorted_by(|a, b| a.start.x.cmp(&b.start.x))
                .filter(|e| e.start.x == e.end.x)
                .for_each(|e| {
                    if e.start.y < y && e.end.y >= y && e.start.x == x {
                        is_inside = !is_inside;
                    }
                });

            if is_inside {
                result.insert(Coord { x, y }, false);
            }
        }
    }

    lagoon.iter().for_each(|e| {
        let ymin = e.start.y.min(e.end.y);
        let ymax = e.start.y.max(e.end.y);
        let xmin = e.start.x.min(e.end.x);
        let xmax = e.start.x.max(e.end.x);
        for y in ymin..=ymax {
            for x in xmin..=xmax {
                result.insert(Coord { x, y }, false);
            }
        }
    });

    Ok(result.len() as i64)
}

// shoelace formula
pub fn part2(input: &str) -> Result<isize> {
    let plan = input.parse::<Plan>()?;
    let mut current = Coord { x: 0, y: 0 };
    let mut lagoon = VecDeque::new();
    for dig in plan.0 {
        let dig = dig.into_color_dig();
        let (mut x, mut y) = (current.x, current.y);
        match dig.direction {
            Direction::Up => y -= dig.len,
            Direction::Down => y += dig.len,
            Direction::Left => x -= dig.len,
            Direction::Right => x += dig.len,
        };
        lagoon.push_back(Coord { x, y });
        current = Coord { x, y };
    }
    let mut area = 0;
    let mut perimeter = 0;
    for i in 0..lagoon.len() {
        let a = lagoon[i];
        let b = lagoon[(i + 1) % lagoon.len()];
        let det = a.x * b.y - a.y * b.x;
        area += det;
        perimeter += (a.x - b.x).abs() + (a.y - b.y).abs();
    }

    Ok(area / 2 + perimeter / 2 + 1)
}

#[derive(Debug)]
struct Plan(Vec<Dig>);

#[derive(Debug)]
struct Edge {
    start: Coord,
    end: Coord,
}

#[derive(Debug)]
struct Dig {
    direction: Direction,
    len: isize,
    color: String,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Ord, PartialOrd)]
struct Coord {
    x: isize,
    y: isize,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => bail!("Invalid direction: {}", s),
        })
    }
}

impl Dig {
    fn commit(&self, start: Coord, lagoon: &mut Vec<Edge>) -> Coord {
        let Coord { mut x, mut y } = start;
        match self.direction {
            Direction::Up => y -= self.len,
            Direction::Down => y += self.len,
            Direction::Left => x -= self.len,
            Direction::Right => x += self.len,
        };
        if start.y < y {
            lagoon.push(Edge {
                start,
                end: Coord { x, y },
            });
        } else {
            lagoon.push(Edge {
                end: start,
                start: Coord { x, y },
            });
        }
        Coord { x, y }
    }
    fn into_color_dig(self) -> Self {
        let mut color = self.color;
        let dir = match color.pop().unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => panic!("Invalid direction: {}", color),
        };
        let len = isize::from_str_radix(&color, 16).unwrap();
        Self {
            direction: dir,
            len,
            color: "".to_string(),
        }
    }
}

impl FromStr for Dig {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some((dir, len)) = s.split_once(' ') else {
            bail!("Invalid direction: {}", s);
        };
        let Some((len, color)) = len.split_once(' ') else {
            bail!("Invalid length: {}", s);
        };

        Ok(Self {
            direction: dir.parse()?,
            len: len.parse()?,
            color: color.trim_start_matches("(#").trim_end_matches(')').to_string(),
        })
    }
}

impl FromStr for Plan {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Plan(s.lines().map(|l| l.parse()).try_collect()?))
    }
}
