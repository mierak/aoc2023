use std::str::FromStr;

use anyhow::{Context, Result};
use itertools::Itertools;

pub fn part1(input: &str) -> Result<f64> {
    Ok(input.parse::<Races>()?.0.iter().fold(1.0, |acc, val| {
        let (x1, x2) = quadratic(1.0, -val.duration, val.record_distance);
        acc * ((x2.ceil() - x1.floor()).abs() + 1.0)
    }))
}

pub fn part2(input: &str) -> Result<f64> {
    part1(&input.replace(' ', ""))
}

fn quadratic(a: f64, b: f64, c: f64) -> (f64, f64) {
    let discr = b.powi(2) - 4.0 * a * c;
    let x1 = (-b + discr.sqrt()) / (2.0 * a);
    let x2 = (-b - discr.sqrt()) / (2.0 * a);
    (x1, x2)
}

#[derive(Debug)]
struct Races(Vec<Race>);

#[derive(Debug)]
struct Race {
    duration: f64,
    record_distance: f64,
}

impl FromStr for Races {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let first = lines
            .next()
            .context("No first line")?
            .split_once("Time:")
            .context("No time")?
            .1;

        let second = lines
            .next()
            .context("No second line")?
            .split_once("Distance:")
            .context("No distance")?
            .1;

        Ok(Races(
            first
                .split_whitespace()
                .zip(second.split_whitespace())
                .map(|(time, distance)| -> Result<Race> {
                    Ok(Race {
                        duration: time.parse::<f64>()?,
                        record_distance: distance.parse::<f64>()?,
                    })
                })
                .try_collect()?,
        ))
    }
}
