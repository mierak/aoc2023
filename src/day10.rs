use std::{collections::VecDeque, str::FromStr};

use anyhow::{anyhow, Result};
use itertools::Itertools;

pub fn part1(input: &str) -> Result<i32> {
    let map = input.parse::<Map>()?;

    let mut len = 1;
    let [mut first, mut second] = map.find_connections_from(&map.start);
    let [mut first_prev, mut second_prev] = [map.start, map.start];
    while first != second {
        let next_first = map.next(&first, &first_prev).ok_or_else(|| anyhow!("no next"))?;
        let next_second = map.next(&second, &second_prev).ok_or_else(|| anyhow!("no next"))?;

        first_prev = first;
        second_prev = second;

        first = next_first;
        second = next_second;

        len += 1;
    }

    Ok(len)
}

pub fn part2(input: &str) -> Result<i32> {
    let mut map = input.parse::<Map>()?;

    let start = map.start;
    let [mut first, mut second] = map.find_connections_from(&start);
    map.mark_as_loop(&first);
    map.mark_as_loop(&second);
    map.mark_as_loop(&start);
    let [mut first_prev, mut second_prev] = [start, start];
    while first != second {
        let next_first = map.next(&first, &first_prev).ok_or_else(|| anyhow!("no next"))?;
        let next_second = map.next(&second, &second_prev).ok_or_else(|| anyhow!("no next"))?;

        first_prev = first;
        second_prev = second;

        first = next_first;
        second = next_second;

        map.mark_as_loop(&first);
        map.mark_as_loop(&second);
    }

    for row in map.map.iter_mut() {
        for pipe in row.iter_mut() {
            if !pipe.is_part_of_loop() {
                *pipe = Pipe::Ground(false);
            }
        }
    }

    let mut expanded = Vec::new();
    for y in 0..map.map.len() {
        let mut row = Vec::new();
        let mut row2 = Vec::new();
        let mut row3 = Vec::new();
        for x in 0..map.map[y].len() {
            match map[Coord { x, y }] {
                Pipe::NorthToSouth(is_pipe) => {
                    row.push(Pipe::Ground(false));
                    row.push(Pipe::NorthToSouth(is_pipe));
                    row.push(Pipe::Ground(false));

                    row2.push(Pipe::Ground(false));
                    row2.push(Pipe::NorthToSouth(is_pipe));
                    row2.push(Pipe::Ground(false));

                    row3.push(Pipe::Ground(false));
                    row3.push(Pipe::NorthToSouth(is_pipe));
                    row3.push(Pipe::Ground(false));
                }
                Pipe::EastToWest(is_pipe) => {
                    row.push(Pipe::Ground(false));
                    row.push(Pipe::Ground(false));
                    row.push(Pipe::Ground(false));

                    row2.push(Pipe::EastToWest(is_pipe));
                    row2.push(Pipe::EastToWest(is_pipe));
                    row2.push(Pipe::EastToWest(is_pipe));

                    row3.push(Pipe::Ground(false));
                    row3.push(Pipe::Ground(false));
                    row3.push(Pipe::Ground(false));
                }
                Pipe::NorthToEast(is_pipe) => {
                    row.push(Pipe::Ground(false));
                    row.push(Pipe::NorthToSouth(is_pipe));
                    row.push(Pipe::Ground(false));

                    row2.push(Pipe::Ground(false));
                    row2.push(Pipe::NorthToEast(is_pipe));
                    row2.push(Pipe::EastToWest(is_pipe));

                    row3.push(Pipe::Ground(false));
                    row3.push(Pipe::Ground(false));
                    row3.push(Pipe::Ground(false));
                }
                Pipe::NorthToWest(is_pipe) => {
                    row.push(Pipe::Ground(false));
                    row.push(Pipe::NorthToSouth(is_pipe));
                    row.push(Pipe::Ground(false));

                    row2.push(Pipe::EastToWest(is_pipe));
                    row2.push(Pipe::NorthToWest(is_pipe));
                    row2.push(Pipe::Ground(false));

                    row3.push(Pipe::Ground(false));
                    row3.push(Pipe::Ground(false));
                    row3.push(Pipe::Ground(false));
                }
                Pipe::SouthToEast(is_pipe) => {
                    row.push(Pipe::Ground(false));
                    row.push(Pipe::Ground(false));
                    row.push(Pipe::Ground(false));

                    row2.push(Pipe::Ground(false));
                    row2.push(Pipe::SouthToEast(is_pipe));
                    row2.push(Pipe::EastToWest(is_pipe));

                    row3.push(Pipe::Ground(false));
                    row3.push(Pipe::NorthToSouth(is_pipe));
                    row3.push(Pipe::Ground(false));
                }
                Pipe::SouthToWest(is_pipe) => {
                    row.push(Pipe::Ground(false));
                    row.push(Pipe::Ground(false));
                    row.push(Pipe::Ground(false));

                    row2.push(Pipe::EastToWest(is_pipe));
                    row2.push(Pipe::SouthToWest(is_pipe));
                    row2.push(Pipe::Ground(false));

                    row3.push(Pipe::Ground(false));
                    row3.push(Pipe::NorthToSouth(is_pipe));
                    row3.push(Pipe::Ground(false));
                }
                Pipe::Ground(_) => {
                    row.push(Pipe::Ground(false));
                    row.push(Pipe::Ground(false));
                    row.push(Pipe::Ground(false));

                    row2.push(Pipe::Ground(false));
                    row2.push(Pipe::Ground(false));
                    row2.push(Pipe::Ground(false));

                    row3.push(Pipe::Ground(false));
                    row3.push(Pipe::Ground(false));
                    row3.push(Pipe::Ground(false));
                }
                Pipe::Start => {
                    let start = Coord { x, y };
                    let [first, second] = map.find_connections_from(&Coord { x, y });

                    row.push(Pipe::Ground(false));
                    row.push(Pipe::Ground(false));
                    row.push(Pipe::Ground(false));

                    row2.push(Pipe::Ground(false));
                    row2.push(Pipe::Start);
                    row2.push(Pipe::Ground(false));

                    row3.push(Pipe::Ground(false));
                    row3.push(Pipe::Ground(false));
                    row3.push(Pipe::Ground(false));

                    let rlen = row.len();
                    let r2len = row2.len();
                    let r3len = row3.len();

                    if first.x == start.x && first.y > start.y {
                        row3[r3len - 2] = Pipe::Start;
                    } else if first.x == start.x && first.y < start.y {
                        row[rlen - 2] = Pipe::Start;
                    } else if first.y == start.y && first.x > start.x {
                        row2[r2len - 1] = Pipe::Start;
                    } else if first.y == start.y && first.x < start.x {
                        row2[r2len - 3] = Pipe::Start;
                    }
                    if second.x == start.x && second.y > start.y {
                        row3[r3len - 2] = Pipe::Start;
                    } else if second.x == start.x && second.y < start.y {
                        row[rlen - 2] = Pipe::Start;
                    } else if second.y == start.y && second.x > start.x {
                        row2[r2len - 1] = Pipe::Start;
                    } else if second.y == start.y && second.x < start.x {
                        row2[r2len - 3] = Pipe::Start;
                    }
                }
            };
        }
        expanded.push(row);
        expanded.push(row2);
        expanded.push(row3);
    }

    let mut exmap = Map {
        start: Coord {
            x: start.x * 3,
            y: start.y * 3,
        },
        map: expanded.clone(),
    };

    fill(Coord { x: 70 * 3, y: 70 * 3 }, &mut exmap);

    let mut count = 0;
    for y in 0..map.map.len() {
        for x in 0..map.map[y].len() {
            if exmap[Coord {
                x: x * 3 + 1,
                y: y * 3 + 1,
            }] == Pipe::Ground(true)
            {
                count += 1;
            }
        }
    }

    Ok(count)
}

fn fill(node: Coord, map: &mut Map) {
    let mut stack = VecDeque::new();
    stack.push_back(node);
    while !stack.is_empty() {
        let current = stack.pop_front().unwrap();
        if map[current] == Pipe::Ground(true) {
            continue;
        }
        map[current] = Pipe::Ground(true);

        if current.up().is_some_and(|up| map[up] == Pipe::Ground(false)) {
            stack.push_back(current.up().unwrap());
        }
        if current.down(map).is_some_and(|down| map[down] == Pipe::Ground(false)) {
            stack.push_back(current.down(map).unwrap());
        }
        if current.left().is_some_and(|left| map[left] == Pipe::Ground(false)) {
            stack.push_back(current.left().unwrap());
        }
        if current
            .right(map)
            .is_some_and(|right| map[right] == Pipe::Ground(false))
        {
            stack.push_back(current.right(map).unwrap());
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Map {
    start: Coord,
    map: Vec<Vec<Pipe>>,
}

impl Map {
    fn next(&self, current: &Coord, previous: &Coord) -> Option<Coord> {
        if current.x > previous.x {
            match self[*current] {
                Pipe::EastToWest(_) => current.right(self),
                Pipe::NorthToWest(_) => current.up(),
                Pipe::SouthToWest(_) => current.down(self),
                _ => None,
            }
        } else if current.x < previous.x {
            match self[*current] {
                Pipe::EastToWest(_) => current.left(),
                Pipe::NorthToEast(_) => current.up(),
                Pipe::SouthToEast(_) => current.down(self),
                _ => None,
            }
        } else if current.y > previous.y {
            match self[*current] {
                Pipe::NorthToSouth(_) => current.down(self),
                Pipe::NorthToEast(_) => current.right(self),
                Pipe::NorthToWest(_) => current.left(),
                _ => None,
            }
        } else if current.y < previous.y {
            match self[*current] {
                Pipe::NorthToSouth(_) => current.up(),
                Pipe::SouthToEast(_) => current.right(self),
                Pipe::SouthToWest(_) => current.left(),
                _ => None,
            }
        } else {
            None
        }
    }

    fn find_connections_from(&self, coord: &Coord) -> [Coord; 2] {
        let mut connections = [Coord { x: 0, y: 0 }, Coord { x: 0, y: 0 }];
        let mut found = 0;
        if let Some(left) = coord.left() {
            match self[left] {
                Pipe::EastToWest(_) | Pipe::NorthToEast(_) | Pipe::SouthToEast(_) => {
                    connections[found] = left;
                    found += 1;
                }
                _ => {}
            };
        }
        if let Some(right) = coord.right(self) {
            match self[right] {
                Pipe::EastToWest(_) | Pipe::NorthToWest(_) | Pipe::SouthToWest(_) => {
                    connections[found] = right;
                    found += 1;
                }
                _ => {}
            };
        }
        if let Some(up) = coord.up() {
            match self[up] {
                Pipe::SouthToEast(_) | Pipe::SouthToWest(_) | Pipe::NorthToSouth(_) => {
                    connections[found] = up;
                    found += 1;
                }
                _ => {}
            };
        }
        if let Some(down) = coord.down(self) {
            match self[down] {
                Pipe::NorthToEast(_) | Pipe::NorthToWest(_) | Pipe::NorthToSouth(_) => {
                    connections[found] = down;
                }
                _ => {}
            };
        }
        connections
    }

    fn mark_as_loop(&mut self, coord: &Coord) {
        self[*coord] = match self[*coord] {
            Pipe::NorthToSouth(_) => Pipe::NorthToSouth(true),
            Pipe::EastToWest(_) => Pipe::EastToWest(true),
            Pipe::NorthToEast(_) => Pipe::NorthToEast(true),
            Pipe::NorthToWest(_) => Pipe::NorthToWest(true),
            Pipe::SouthToEast(_) => Pipe::SouthToEast(true),
            Pipe::SouthToWest(_) => Pipe::SouthToWest(true),
            Pipe::Ground(f) => Pipe::Ground(f),
            Pipe::Start => Pipe::Start,
        };
    }
}

impl std::ops::Index<Coord> for Map {
    type Output = Pipe;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.map[index.y][index.x]
    }
}

impl std::ops::IndexMut<Coord> for Map {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.map[index.y][index.x]
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = Coord { x: 0, y: 0 };
        let map = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '|' => Pipe::NorthToSouth(false),
                        '-' => Pipe::EastToWest(false),
                        'L' => Pipe::NorthToEast(false),
                        'F' => Pipe::SouthToEast(false),
                        'J' => Pipe::NorthToWest(false),
                        '7' => Pipe::SouthToWest(false),
                        '.' => Pipe::Ground(false),
                        'S' => {
                            start.x = x;
                            start.y = y;
                            Pipe::Start
                        }
                        _ => unreachable!(),
                    })
                    .collect_vec()
            })
            .collect_vec();
        Ok(Map { map, start })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    NorthToSouth(bool),
    EastToWest(bool),
    NorthToEast(bool),
    NorthToWest(bool),
    SouthToEast(bool),
    SouthToWest(bool),
    Ground(bool),
    Start,
}

impl Pipe {
    fn is_part_of_loop(&self) -> bool {
        matches!(
            self,
            Pipe::NorthToSouth(true)
                | Pipe::EastToWest(true)
                | Pipe::NorthToEast(true)
                | Pipe::NorthToWest(true)
                | Pipe::SouthToEast(true)
                | Pipe::SouthToWest(true)
                | Pipe::Start
        )
    }
}

impl std::fmt::Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pipe::NorthToSouth(_) => write!(f, "┃"),
            Pipe::EastToWest(_) => write!(f, "━"),
            Pipe::NorthToEast(_) => write!(f, "┗"),
            Pipe::NorthToWest(_) => write!(f, "┛"),
            Pipe::SouthToEast(_) => write!(f, "┏"),
            Pipe::SouthToWest(_) => write!(f, "┓"),
            Pipe::Ground(true) => write!(f, "o"),
            Pipe::Ground(false) => write!(f, "."),
            Pipe::Start => write!(f, "S"),
        }
    }
}

impl Coord {
    fn left(&self) -> Option<Coord> {
        if self.x == 0 {
            None
        } else {
            Some(Coord {
                x: self.x - 1,
                y: self.y,
            })
        }
    }

    fn right(&self, map: &Map) -> Option<Coord> {
        if self.x == map.map[0].len() - 1 {
            None
        } else {
            Some(Coord {
                x: self.x + 1,
                y: self.y,
            })
        }
    }

    fn up(&self) -> Option<Coord> {
        if self.y == 0 {
            None
        } else {
            Some(Coord {
                x: self.x,
                y: self.y - 1,
            })
        }
    }

    fn down(&self, map: &Map) -> Option<Coord> {
        if self.y == map.map.len() - 1 {
            None
        } else {
            Some(Coord {
                x: self.x,
                y: self.y + 1,
            })
        }
    }
}
