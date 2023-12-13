use std::str::FromStr;

use anyhow::{Context, Result};
use itertools::Itertools;

pub fn part1(input: &str) -> Result<usize> {
    let grids: Grids = input.parse()?;

    let mut result = 0;
    for mut grid in grids.0 {
        for row_idx in 0..grid.data.len() - 1 {
            if grid.mirrored_at_rows(row_idx, row_idx + 1) == 0 {
                result += 100 * (row_idx + 1);
            }
        }

        for col_idx in 0..grid.data[0].len() - 1 {
            if grid.mirrored_at_cols(col_idx, col_idx + 1) == 0 {
                result += col_idx + 1;
            }
        }
    }

    Ok(result)
}

pub fn part2(input: &str) -> Result<usize> {
    let grids: Grids = input.parse()?;

    let mut result = 0;
    for mut grid in grids.0 {
        for row_idx in 0..grid.data.len() - 1 {
            if grid.mirrored_at_rows(row_idx, row_idx + 1) == 1 {
                result += 100 * (row_idx + 1);
            }
        }

        for col_idx in 0..grid.data[0].len() - 1 {
            if grid.mirrored_at_cols(col_idx, col_idx + 1) == 1 {
                result += col_idx + 1;
            }
        }
    }

    Ok(result)
}

struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {
    fn mirrored_at_rows(&mut self, top: usize, bottom: usize) -> i32 {
        let mut top = top;
        let mut bottom = bottom;

        let mut differences = 0;
        loop {
            let result = self.row_eq(top, bottom);
            if result != 0 {
                differences += result;
            }
            if top == 0 || bottom == self.data.len() - 1 {
                break;
            }

            top -= 1;
            bottom += 1;
        }

        differences
    }

    fn mirrored_at_cols(&mut self, left: usize, right: usize) -> i32 {
        let mut left = left;
        let mut right = right;

        let mut differences = 0;
        loop {
            let result = self.col_eq(left, right);
            if result != 0 {
                differences += result;
            }

            if left == 0 || right == self.data[0].len() - 1 {
                break;
            }
            left -= 1;
            right += 1;
        }

        differences
    }

    fn row_eq(&mut self, row_idx: usize, other_row_idx: usize) -> i32 {
        self.data[row_idx]
            .iter()
            .zip(self.data[other_row_idx].iter())
            .filter(|(a, b)| a != b)
            .count() as i32
    }

    fn col_eq(&mut self, col_idx: usize, other_col_idx: usize) -> i32 {
        self.data
            .iter()
            .filter(|row| row[col_idx] != row[other_col_idx])
            .count() as i32
    }
}

struct Grids(Vec<Grid>);

impl FromStr for Grids {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self> {
        Ok(Grids(
            input
                .split("\n\n")
                .map(|s| s.parse().context("failed to parse grid"))
                .try_collect()?,
        ))
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self> {
        Ok(Grid {
            data: input.lines().map(|l| l.chars().collect_vec()).collect_vec(),
        })
    }
}
