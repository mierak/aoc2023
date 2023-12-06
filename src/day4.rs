use std::{collections::VecDeque, str::FromStr};

use anyhow::{bail, Result};
use itertools::Itertools;

pub fn part1(input: &str) -> Result<i32> {
    input
        .lines()
        .map(|line| line.parse::<Card>())
        .map(|card| -> Result<i32> { Ok(card?.count_points()) })
        .try_fold(0, |acc, val| -> Result<i32> { Ok(acc + val?) })
}
pub fn part2(input: &str) -> Result<i32> {
    let original_cards: VecDeque<_> = input.lines().map(|line| line.parse::<Card>()).try_collect()?;
    let mut processed_cards = 0;

    let mut current_cards: VecDeque<_> = (0..original_cards.len()).collect();
    while !current_cards.is_empty() {
        let current_card = current_cards
            .pop_front()
            .expect("There to be a card as it was checked before");
        let current_card = &original_cards[current_card];
        let matching = current_card.count_matching();

        (current_card.idx + 1..current_card.idx + matching + 1).for_each(|i| {
            current_cards.push_back(i);
        });

        processed_cards += 1;
    }

    Ok(processed_cards)
}

#[derive(Debug, Default, Clone)]
struct Card {
    idx: usize,
    winning_nums: Vec<i32>,
    rolled_nums: Vec<i32>,
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some((num, rest)) = s.split_once(": ") else {
            bail!("Invalid card format")
        };
        let Some((winning_nums, rolled_nums)) = rest.split_once(" | ") else {
            bail!("Invalid card format")
        };

        let mut res = Card {
            idx: num.trim_start_matches("Card").trim_start().parse::<usize>()? - 1,
            winning_nums: Vec::new(),
            rolled_nums: Vec::new(),
        };

        for ele in winning_nums.split_whitespace() {
            res.winning_nums.push(ele.parse()?);
        }
        for ele in rolled_nums.split_whitespace() {
            res.rolled_nums.push(ele.parse()?);
        }
        Ok(res)
    }
}

impl Card {
    fn count_points(&self) -> i32 {
        let mut res = 1;
        for ele in self.winning_nums.iter() {
            if self.rolled_nums.contains(ele) {
                res *= 2;
            }
        }
        res / 2
    }

    fn count_matching(&self) -> usize {
        self.rolled_nums
            .iter()
            .filter(|ele| self.winning_nums.contains(ele))
            .count()
    }
}
