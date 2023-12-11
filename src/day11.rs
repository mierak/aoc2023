use std::str::FromStr;

use anyhow::Result;
use itertools::Itertools;

pub fn part1(input: &str) -> Result<i64> {
    let mut space = Space::from_str(input)?;
    let expansion = 2;
    space.calc_expansion_factors();

    Ok(space.galaxies.iter().combinations(2).fold(0, |acc, val| {
        acc + val[0]
            .apply_multiplier(expansion)
            .dist(&val[1].apply_multiplier(expansion))
    }))
}

pub fn part2(input: &str) -> Result<i64> {
    let mut space = Space::from_str(input)?;
    let expansion = 1_000_000;
    space.calc_expansion_factors();

    Ok(space.galaxies.iter().combinations(2).fold(0, |acc, val| {
        acc + val[0]
            .apply_multiplier(expansion)
            .dist(&val[1].apply_multiplier(expansion))
    }))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord {
    x: usize,
    y: usize,
    x_multipier: usize,
    y_multipier: usize,
}

impl Coord {
    fn dist(&self, other: &Self) -> i64 {
        (self.x as i64 - other.x as i64).abs() + (self.y as i64 - other.y as i64).abs()
    }
    fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            x_multipier: 0,
            y_multipier: 0,
        }
    }
    fn apply_multiplier(&self, expansion_factor: usize) -> Self {
        let mut x = self.x;
        let mut y = self.y;
        if self.x_multipier > 0 {
            x += (expansion_factor * self.x_multipier) - self.x_multipier;
        }
        if self.y_multipier > 0 {
            y += (expansion_factor * self.y_multipier) - self.y_multipier;
        }

        Self::new(x, y)
    }
}

#[derive(Debug)]
struct Space {
    galaxies: Vec<Coord>,
}

impl Space {
    fn size(&self) -> (usize, usize) {
        let mut max_x = 0;
        let mut max_y = 0;
        for galaxy in &self.galaxies {
            if galaxy.x > max_x {
                max_x = galaxy.x;
            }
            if galaxy.y > max_y {
                max_y = galaxy.y;
            }
        }
        (max_y + 1, max_x + 1)
    }

    fn is_row_empty(&self, row: usize) -> bool {
        !self.galaxies.iter().any(|g| g.y == row.try_into().unwrap())
    }

    fn is_col_empty(&self, col: usize) -> bool {
        !self.galaxies.iter().any(|g| g.x == col.try_into().unwrap())
    }

    fn calc_expansion_factors(&mut self) {
        let start = std::time::Instant::now();
        let (rows, cols) = self.size();
        let mut empty_rows_reached = 0;
        let empty_cols = (0..cols).map(|x| self.is_col_empty(x)).collect_vec();
        for y in 0..rows {
            if self.is_row_empty(y) {
                empty_rows_reached += 1;
                continue;
            }

            let mut empty_cols_reached = 0;
            for x in 0..cols {
                if empty_cols[x] {
                    empty_cols_reached += 1;
                    continue;
                }

                if let Some(coord) = self.galaxies.iter_mut().find(|g| g.x == x && g.y == y) {
                    if empty_cols_reached > 0 {
                        coord.x_multipier = empty_cols_reached;
                    }
                    if empty_rows_reached > 0 {
                        coord.y_multipier = empty_rows_reached;
                    }
                }
            }
        }
        println!("Expand took {:?}", start.elapsed());
    }
}

impl FromStr for Space {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let galaxies = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| match c {
                    '#' => Some(Coord::new(x, y)),
                    _ => None,
                })
            })
            .collect_vec();

        Ok(Space { galaxies })
    }
}
