use anyhow::Result;
use itertools::Itertools;
use std::{collections::HashMap, str::FromStr};

pub fn part1(input: &str) -> Result<i64> {
    Ok(input.parse::<Grid>()?.energize(Coord { x: 0, y: 0 }, Direction::Right))
}

pub fn part2(input: &str) -> Result<i64> {
    let mut grid = input.parse::<Grid>()?;
    let grid_len = grid.0[0].len() - 1;

    Ok((0..grid.0.len()).fold(0, |acc, i| {
        [
            (Coord { x: 0, y: i }, Direction::Right),
            (Coord { x: grid_len, y: i }, Direction::Left),
            (Coord { x: i, y: 0 }, Direction::Down),
            (Coord { x: i, y: grid_len }, Direction::Up),
        ]
        .iter()
        .fold(0, |acc, (coord, direction)| {
            grid.reset();
            grid.energize(*coord, *direction).max(acc)
        })
        .max(acc)
    }))
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn move_in_direction(&self, direction: &Direction, grid: &Grid) -> Option<Self> {
        match direction {
            Direction::Up if self.y > 0 => Some(Self {
                x: self.x,
                y: self.y - 1,
            }),
            Direction::Down if self.y < grid.0.len() - 1 => Some(Self {
                x: self.x,
                y: self.y + 1,
            }),
            Direction::Left if self.x > 0 => Some(Self {
                x: self.x - 1,
                y: self.y,
            }),
            Direction::Right if self.x < grid.0[0].len() - 1 => Some(Self {
                x: self.x + 1,
                y: self.y,
            }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct Grid(Vec<Vec<Tile>>);

impl Grid {
    fn reset(&mut self) {
        self.0.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|tile| match tile {
                Tile::MirrorUp { energized: has_beam } => *has_beam = false,
                Tile::MirrorDown { energized: has_beam } => *has_beam = false,
                Tile::SplitterVertical { energized: has_beam } => *has_beam = false,
                Tile::SplitterHorizontal { energized: has_beam } => *has_beam = false,
                Tile::Empty { energized: has_beam } => *has_beam = false,
            })
        })
    }

    fn energize(&mut self, position: Coord, direction: Direction) -> i64 {
        self.traverse(position, direction, &mut HashMap::new())
    }

    fn traverse(
        &mut self,
        position: Coord,
        current_source_direction: Direction,
        cache: &mut HashMap<(Direction, Coord), ()>,
    ) -> i64 {
        if cache.contains_key(&(current_source_direction, position)) {
            return 0;
        }
        cache.insert((current_source_direction, position), ());

        let mut result = 0;
        let tile = &mut self[&position];
        if !tile.is_energized() {
            result += 1;
        }
        tile.energize();

        match tile.next(&current_source_direction) {
            (direction1, None) => position
                .move_in_direction(&direction1, self)
                .map_or(result, |pos| result + self.traverse(pos, direction1, cache)),
            (direction1, Some(direction2)) => {
                match (
                    position.move_in_direction(&direction1, self),
                    position.move_in_direction(&direction2, self),
                ) {
                    (Some(pos1), Some(pos2)) => {
                        result + self.traverse(pos1, direction1, cache) + self.traverse(pos2, direction2, cache)
                    }
                    (Some(pos1), None) => self.traverse(pos1, direction1, cache),
                    (None, Some(pos2)) => result + self.traverse(pos2, direction2, cache),
                    (None, None) => result,
                }
            }
        }
    }
}

impl std::ops::IndexMut<&Coord> for Grid {
    fn index_mut(&mut self, index: &Coord) -> &mut Tile {
        &mut self.0[index.y][index.x]
    }
}

impl std::ops::Index<&Coord> for Grid {
    type Output = Tile;

    fn index(&self, index: &Coord) -> &Self::Output {
        &self.0[index.y][index.x]
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Grid(
            s.lines()
                .map(|line| line.chars().map(|c| c.try_into()).try_collect())
                .try_collect()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    MirrorUp { energized: bool },
    MirrorDown { energized: bool },
    SplitterVertical { energized: bool },
    SplitterHorizontal { energized: bool },
    Empty { energized: bool },
}

impl Tile {
    fn next(&self, source_direction: &Direction) -> (Direction, Option<Direction>) {
        match (self, source_direction) {
            (Tile::MirrorUp { .. }, Direction::Up) => (Direction::Right, None),
            (Tile::MirrorUp { .. }, Direction::Down) => (Direction::Left, None),
            (Tile::MirrorUp { .. }, Direction::Left) => (Direction::Down, None),
            (Tile::MirrorUp { .. }, Direction::Right) => (Direction::Up, None),
            (Tile::MirrorDown { .. }, Direction::Up) => (Direction::Left, None),
            (Tile::MirrorDown { .. }, Direction::Down) => (Direction::Right, None),
            (Tile::MirrorDown { .. }, Direction::Left) => (Direction::Up, None),
            (Tile::MirrorDown { .. }, Direction::Right) => (Direction::Down, None),
            (Tile::SplitterVertical { .. }, Direction::Up) => (Direction::Up, None),
            (Tile::SplitterVertical { .. }, Direction::Down) => (Direction::Down, None),
            (Tile::SplitterVertical { .. }, Direction::Left) => (Direction::Up, Some(Direction::Down)),
            (Tile::SplitterVertical { .. }, Direction::Right) => (Direction::Up, Some(Direction::Down)),
            (Tile::SplitterHorizontal { .. }, Direction::Up) => (Direction::Left, Some(Direction::Right)),
            (Tile::SplitterHorizontal { .. }, Direction::Down) => (Direction::Left, Some(Direction::Right)),
            (Tile::SplitterHorizontal { .. }, Direction::Left) => (Direction::Left, None),
            (Tile::SplitterHorizontal { .. }, Direction::Right) => (Direction::Right, None),
            _ => (*source_direction, None),
        }
    }

    fn energize(&mut self) {
        match self {
            Tile::MirrorUp { energized: has_beam } => *has_beam = true,
            Tile::MirrorDown { energized: has_beam } => *has_beam = true,
            Tile::SplitterVertical { energized: has_beam } => *has_beam = true,
            Tile::SplitterHorizontal { energized: has_beam } => *has_beam = true,
            Tile::Empty { energized: has_beam } => *has_beam = true,
        }
    }

    fn is_energized(&self) -> bool {
        match self {
            Tile::MirrorUp { energized: has_beam } => *has_beam,
            Tile::MirrorDown { energized: has_beam } => *has_beam,
            Tile::SplitterVertical { energized: has_beam } => *has_beam,
            Tile::SplitterHorizontal { energized: has_beam } => *has_beam,
            Tile::Empty { energized: has_beam } => *has_beam,
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;
    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            '|' => Ok(Tile::SplitterVertical { energized: false }),
            '-' => Ok(Tile::SplitterHorizontal { energized: false }),
            '/' => Ok(Tile::MirrorUp { energized: false }),
            '\\' => Ok(Tile::MirrorDown { energized: false }),
            '.' => Ok(Tile::Empty { energized: false }),
            _ => Err(anyhow::anyhow!("Invalid tile: {}", s)),
        }
    }
}
