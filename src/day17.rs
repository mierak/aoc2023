use anyhow::{Context, Result};
use itertools::Itertools;
use std::{
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

pub fn part1(input: &str) -> Result<i64> {
    let mut graph = input.parse::<Graph>()?;
    let start = Coord { x: 0, y: 0 };
    let end = Coord {
        x: graph.0.last().unwrap().len() - 1,
        y: graph.0.len() - 1,
    };

    Ok(graph.solve(start, end, 0, 3) / 2)
}

pub fn part2(input: &str) -> Result<i64> {
    let mut graph = input.parse::<Graph>()?;
    let start = Coord { x: 0, y: 0 };
    let end = Coord {
        x: graph.0.last().unwrap().len() - 1,
        y: graph.0.len() - 1,
    };

    Ok(graph.solve(start, end, 4, 10) / 2)
}

#[rustfmt::skip]
impl Graph {
    fn solve(&mut self, start: Coord, end: Coord, min_steps: i64, max_steps: i64) -> i64 {
        let mut visited = HashMap::new();
        let mut stack = BinaryHeap::new();

        visited.insert(VisitedKey { position: start, direction: Direction::Right, steps_in_direction: 0 }, 0);
        visited.insert(VisitedKey { position: start, direction: Direction::Down, steps_in_direction: 0, }, 0);
        stack.push(Frame { position: start, direction: Direction::Right, cost: 0, steps_in_direction: 0 });
        while let Some(Frame { direction, position, cost, steps_in_direction }) = stack.pop()
        {
            if position == end {
                return cost;
            }

            let dist_key = VisitedKey {
                position,
                direction,
                steps_in_direction,
            };

            if visited.contains_key(&dist_key) && visited[&dist_key] < cost {
                continue;
            }

            for c in [
                (position.above(self), Direction::Up),
                (position.right(self), Direction::Right),
                (position.below(self), Direction::Down),
                (position.left(self), Direction::Left),
            ]
            .iter()
            .filter(|c| direction != c.1.opposite())
            .filter_map(|(pos, dir)| pos.map(|p| (p, dir)))
            {
                let (new_pos, direction2) = c;

                let cost = cost + self[new_pos];

                let next_frame = Frame {
                    position: new_pos,
                    direction: *direction2,
                    cost: cost + self[new_pos],
                    steps_in_direction: if direction == *direction2 {
                        steps_in_direction + 1
                    } else {
                        1
                    },
                };

                let dist_key = VisitedKey {
                    position: next_frame.position,
                    direction: next_frame.direction,
                    steps_in_direction: next_frame.steps_in_direction,
                };

                if (direction == *direction2 || steps_in_direction >= min_steps)
                    && next_frame.steps_in_direction <= max_steps
                    && (!visited.contains_key(&dist_key) || next_frame.cost < visited[&dist_key])
                {
                    stack.push(next_frame);
                    visited.insert(dist_key, next_frame.cost);
                }
            }
        }
        unreachable!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct VisitedKey {
    direction: Direction,
    position: Coord,
    steps_in_direction: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Frame {
    direction: Direction,
    position: Coord,
    cost: i64,
    steps_in_direction: i64,
}

impl std::cmp::Ord for Frame {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl std::cmp::PartialOrd for Frame {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<Frame> for VisitedKey {
    fn from(f: Frame) -> Self {
        Self {
            direction: f.direction,
            position: f.position,
            steps_in_direction: f.steps_in_direction,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Ord, PartialOrd)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn above(&self, _g: &Graph) -> Option<Self> {
        if self.y == 0 {
            return None;
        }
        Some(Self {
            x: self.x,
            y: self.y - 1,
        })
    }
    fn right(&self, g: &Graph) -> Option<Self> {
        if self.x == g.0[self.y].len() - 1 {
            return None;
        }
        Some(Self {
            x: self.x + 1,
            y: self.y,
        })
    }
    fn below(&self, g: &Graph) -> Option<Self> {
        if self.y == g.0.len() - 1 {
            return None;
        }
        Some(Self {
            x: self.x,
            y: self.y + 1,
        })
    }
    fn left(&self, _g: &Graph) -> Option<Self> {
        if self.x == 0 {
            return None;
        }
        Some(Self {
            x: self.x - 1,
            y: self.y,
        })
    }
}

struct Graph(Vec<Vec<i64>>);

impl std::ops::Index<Coord> for Graph {
    type Output = i64;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.0[index.y][index.x]
    }
}

impl std::ops::IndexMut<Coord> for Graph {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.0[index.y][index.x]
    }
}

impl FromStr for Graph {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Graph(
            s.lines()
                .map(|line| {
                    line.chars()
                        .map(|s| s.to_digit(10).map(|v| v.try_into().unwrap()).context("Invalid digit"))
                        .try_collect()
                })
                .try_collect()?,
        ))
    }
}
