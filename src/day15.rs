use std::str::FromStr;

use anyhow::{Context, Result};
use itertools::Itertools;

pub fn part1(input: &str) -> Result<usize> {
    Ok(input
        .split(',')
        .map(|v| v.trim().chars().fold(0, |acc, val| ((acc + val as usize) * 17) % 256))
        .sum::<usize>())
}

pub fn part2(input: &str) -> Result<usize> {
    Ok(input
        .split(',')
        .filter_map(|v| -> Option<Operation> { v.parse::<Operation>().ok() })
        .fold((0..256).map(|_| Vec::<Lens>::new()).collect_vec(), |mut acc, val| {
            match val {
                Operation {
                    label,
                    operation_type: OperationType::Remove,
                    box_idx,
                } => {
                    if let Some((idx, _)) = acc[box_idx].iter().enumerate().find(|(_, v)| v.label == label) {
                        acc[box_idx].remove(idx);
                    }
                }
                Operation {
                    ref label,
                    operation_type: OperationType::Insert { focal_length },
                    box_idx,
                } => {
                    if let Some(ref mut lens) = acc[box_idx].iter_mut().find(|v| &v.label == label) {
                        lens.focal_length = focal_length;
                    } else {
                        acc[box_idx].push(Lens {
                            label: label.clone(),
                            focal_length,
                        });
                    }
                }
            }
            acc
        })
        .iter()
        .enumerate()
        .filter(|(_, v)| !v.is_empty())
        .map(|(box_idx, v)| {
            v.iter().enumerate().fold(0, |acc, (idx, val)| {
                acc + (box_idx + 1) * ((idx + 1) * val.focal_length)
            })
        })
        .sum::<usize>())
}

struct Lens {
    label: String,
    focal_length: usize,
}

#[derive(Debug, Default)]
enum OperationType {
    #[default]
    Remove,
    Insert {
        focal_length: usize,
    },
}
#[derive(Debug, Default)]
struct Operation {
    box_idx: usize,
    operation_type: OperationType,
    label: String,
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.trim()
            .chars()
            .try_fold(Operation::default(), |mut acc, val| -> Result<Operation> {
                match val {
                    c if c.is_alphabetic() => {
                        acc.label.push(c);
                        acc.box_idx = ((acc.box_idx + val as usize) * 17) % 256;
                    }
                    '-' => acc.operation_type = OperationType::Remove,
                    '=' => acc.operation_type = OperationType::Insert { focal_length: 0 },
                    num if num.is_numeric() && matches!(acc.operation_type, OperationType::Insert { .. }) => {
                        acc.operation_type = OperationType::Insert {
                            focal_length: num.to_digit(10).context("Invalid number")? as usize,
                        }
                    }
                    _ => {}
                }
                Ok(acc)
            })
    }
}
