use std::{collections::HashMap, str::FromStr};

use anyhow::{Context, Result};
use itertools::Itertools;

pub fn part1(input: &str) -> Result<usize> {
    let mut binding = input.parse::<Springs>()?;
    let results = binding
        .0
        .iter_mut()
        .fold(0, |acc, val| acc + val.find_valid_positions(0, 0));

    Ok(results)
}
pub fn part2(input: &str) -> Result<usize> {
    let mut binding = input.parse::<Springs>()?;

    // expand the input
    for row in binding.0.iter_mut() {
        let lengths = row.lengths.clone();
        let spring = row.springs.clone();
        for _ in 0..4 {
            row.lengths.extend_from_slice(&lengths);
            row.springs.push(Spring::Unknown);
            row.springs.extend_from_slice(&spring);
        }
    }

    let results = binding
        .0
        .iter_mut()
        .fold(0, |acc, val| acc + val.find_valid_positions(0, 0));

    Ok(results)
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Spring {
    Damaged,
    Unknown,
    Working,
}

struct SpringsRow {
    springs: Vec<Spring>,
    lengths: Vec<usize>,
    memo: HashMap<(usize, usize), usize>,
}

impl SpringsRow {
    fn find_valid_positions(&mut self, length_idx: usize, start_idx: usize) -> usize {
        let length = self.lengths[length_idx];
        let mut result = 0;

        // Already solved, return cached result
        if self.memo.contains_key(&(length_idx, start_idx)) {
            return self.memo[&(length_idx, start_idx)];
        }

        // If we are at the end of the lengths, we have a valid solution
        if length == self.springs.len() {
            return 1;
        }

        for i in start_idx..self.springs.len() {
            // Section is longer than reamining springs, nothing to do anymore
            if i + length > self.springs.len() {
                break;
            }

            // Working spring in the range we are checking, skip
            if self.springs[i..i + length].iter().any(|v| *v == Spring::Working) {
                continue;
            }

            // Damaged spring was skipped, no other solution from this attempt is valid
            if self.springs[start_idx..i].iter().any(|v| *v == Spring::Damaged) {
                break;
            }

            // Next neighbour is damaged spring, we need a space here, skip
            if i + length < self.springs.len() && (self.springs[i + length] == Spring::Damaged) {
                continue;
            }

            // If we are at the last section, check if it is valid
            // Otherwise, continue recursively with the next section
            if length_idx == self.lengths.len() - 1 {
                if self.springs.iter().skip(i + 1 + length).any(|v| *v == Spring::Damaged) {
                    continue;
                }
                // Cache result for this section of the row
                *self.memo.entry((length_idx, start_idx)).or_insert(0) += 1;
                result += 1;
            } else {
                let res = self.find_valid_positions(length_idx + 1, i + length + 1);
                // Cache result for this section of the row
                *self.memo.entry((length_idx, start_idx)).or_insert(0) += res;
                result += res;
            }
        }
        result
    }
}

struct Springs(Vec<SpringsRow>);

impl FromStr for Springs {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        s.lines()
            .map(|l| -> Result<SpringsRow> {
                let (springs, lenghts) = l.split_once(' ').context("Invalid input")?;
                let springs = springs
                    .chars()
                    .map(|c| match c {
                        '#' => Spring::Damaged,
                        '?' => Spring::Unknown,
                        '.' => Spring::Working,
                        _ => panic!("Invalid input"),
                    })
                    .collect_vec();
                let lengths = lenghts.split(',').map(|l| l.parse::<usize>()).try_collect()?;
                Ok(SpringsRow {
                    springs,
                    lengths,
                    memo: HashMap::new(),
                })
            })
            .try_collect()
            .map(Springs)
    }
}
