use std::str::FromStr;

use anyhow::{Context, Result};
use itertools::Itertools;

pub fn part1(input: &str) -> Result<i32> {
    input
        .parse::<Items>()?
        .0
        .into_iter()
        .map(|item| -> Result<i32> { Ok(item.fill()?.extrapolate_right()) })
        .try_fold(0, |a, b| b.map(|b| b + a))
}

pub fn part2(input: &str) -> Result<i32> {
    input
        .parse::<Items>()?
        .0
        .into_iter()
        .map(|item| -> Result<i32> { Ok(item.fill()?.extrapolate_left()) })
        .try_fold(0, |a, b| b.map(|b| b + a))
}

#[derive(Debug)]
struct Items(Vec<Item>);

#[derive(Debug)]
struct Item {
    sequences: Vec<Vec<i32>>,
}

impl Item {
    fn fill(mut self) -> Result<Self> {
        while !self.sequences.last().is_some_and(|last| last.iter().all(|v| v == &0)) {
            self.sequences.push(
                self.sequences
                    .last()
                    .context("There to be at least one sequence")?
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| (b - a))
                    .collect(),
            );
        }

        Ok(self)
    }

    fn extrapolate_right(&self) -> i32 {
        let mut to_add = None;
        for i in (0..self.sequences.len()).rev() {
            if to_add.is_none() {
                to_add = self.sequences[i].last().copied();
            } else {
                to_add = to_add.and_then(|to_add| self.sequences[i].last().as_ref().map(|v| *v + to_add))
            }
        }
        to_add.unwrap()
    }

    fn extrapolate_left(&self) -> i32 {
        let mut to_add_left = None;
        for i in (0..self.sequences.len()).rev() {
            if to_add_left.is_none() {
                to_add_left = self.sequences[i].first().copied();
            } else {
                to_add_left =
                    to_add_left.and_then(|to_add_left| self.sequences[i].first().as_ref().map(|v| *v - to_add_left))
            }
        }
        to_add_left.unwrap()
    }
}

impl FromStr for Items {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Items(
            s.lines()
                .map(|line| -> Result<Item> {
                    Ok(Item {
                        sequences: vec![line.split(' ').map(|val| val.parse::<i32>()).try_collect()?],
                    })
                })
                .try_collect()?,
        ))
    }
}
