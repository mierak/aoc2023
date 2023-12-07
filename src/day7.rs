use anyhow::{anyhow, bail, Context, Result};
use itertools::Itertools;

pub fn part1(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(|line| Part1(line).try_into().expect("Parsing to always succeed"))
        .sorted()
        .rev()
        .enumerate()
        .fold(0, |acc, (idx, hand): (usize, HandOfCards)| acc + hand.bid * (idx + 1)))
}

pub fn part2(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(|line| Part2(line).try_into().expect("Parsing to always succeed"))
        .sorted()
        .rev()
        .enumerate()
        .fold(0, |acc, (idx, hand): (usize, HandOfCards)| acc + hand.bid * (idx + 1)))
}

#[derive(Eq, PartialEq, Debug)]
struct HandOfCards {
    cards: [usize; 5],
    bid: usize,
    hand_type: HandType,
    test: Option<Vec<usize>>,
}

struct Part1<'a>(&'a str);
struct Part2<'a>(&'a str);

impl TryFrom<Part1<'_>> for HandOfCards {
    type Error = anyhow::Error;

    fn try_from(s: Part1) -> Result<Self, Self::Error> {
        let (cards, bid) = s.0.split_once(' ').context(anyhow!("No bid found: {:?}", s.0))?;
        let cards = cards
            .chars()
            .enumerate()
            .try_fold([0; 5], |mut acc, (idx, val)| -> Result<[usize; 5]> {
                acc[idx] = Part1::card_value(val)?;
                Ok(acc)
            })?;
        let hand_type = match cards
            .iter()
            .fold([0; 15], |mut acc, val| {
                acc[*val] += 1;
                acc
            })
            .into_iter()
            .sorted()
            .rev()
            .collect_vec()
            .as_slice()
        {
            [5, ..] => Ok(HandType::FiveOfAKind),
            [4, 1, ..] => Ok(HandType::FourOfAKind),
            [3, 2, ..] => Ok(HandType::FullHouse),
            [3, 1, ..] => Ok(HandType::ThreeOfAKind),
            [2, 2, ..] => Ok(HandType::TwoPair),
            [2, 1, ..] => Ok(HandType::OnePair),
            [1, 1, ..] => Ok(HandType::HighCard),
            v => Err(anyhow!("Invalid hand {:?}", v)),
        }?;

        Ok(Self {
            cards,
            bid: bid.parse()?,
            hand_type,
            test: None,
        })
    }
}

impl TryFrom<Part2<'_>> for HandOfCards {
    type Error = anyhow::Error;

    fn try_from(s: Part2) -> Result<Self, Self::Error> {
        let (cards, bid) = s.0.split_once(' ').context(anyhow!("No bid found: {:?}", s.0))?;
        let cards = cards
            .chars()
            .enumerate()
            .try_fold([0; 5], |mut acc, (idx, val)| -> Result<[usize; 5]> {
                acc[idx] = Part2::card_value(val)?;
                Ok(acc)
            })?;
        let mut hand_type = cards.iter().fold([0; 15], |mut acc, val| {
            acc[*val] += 1;
            acc
        });
        let joker_count = hand_type[1];
        hand_type[1] = 0;
        let mut hand_types = hand_type.into_iter().sorted().rev().collect_vec();
        *hand_types.first_mut().context("No first element")? += joker_count;
        let hand_type = match hand_types.as_slice() {
            [5, ..] => Ok(HandType::FiveOfAKind),
            [4, 1, ..] => Ok(HandType::FourOfAKind),
            [3, 2, ..] => Ok(HandType::FullHouse),
            [3, 1, ..] => Ok(HandType::ThreeOfAKind),
            [2, 2, ..] => Ok(HandType::TwoPair),
            [2, 1, ..] => Ok(HandType::OnePair),
            [1, 1, ..] => Ok(HandType::HighCard),
            v => Err(anyhow!("Invalid hand {:?}", v)),
        }?;

        Ok(Self {
            cards,
            bid: bid.parse()?,
            hand_type,
            test: Some(hand_types),
        })
    }
}

impl Ord for HandOfCards {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type == other.hand_type {
            for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                if self_card != other_card {
                    return other_card.cmp(self_card);
                } else {
                    continue;
                }
            }
            std::cmp::Ordering::Equal
        } else {
            self.hand_type.cmp(&other.hand_type)
        }
    }
}

impl PartialOrd for HandOfCards {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

trait CardValue {
    fn card_value(c: char) -> Result<usize>;
}

impl CardValue for Part1<'_> {
    fn card_value(c: char) -> Result<usize> {
        match c {
            'A' => Ok(14),
            'K' => Ok(13),
            'Q' => Ok(12),
            'J' => Ok(11),
            'T' => Ok(10),
            '9' => Ok(9),
            '8' => Ok(8),
            '7' => Ok(7),
            '6' => Ok(6),
            '5' => Ok(5),
            '4' => Ok(4),
            '3' => Ok(3),
            '2' => Ok(2),
            _ => bail!("Invalid card value"),
        }
    }
}

impl CardValue for Part2<'_> {
    fn card_value(c: char) -> Result<usize> {
        match c {
            'A' => Ok(14),
            'K' => Ok(13),
            'Q' => Ok(12),
            'J' => Ok(1),
            'T' => Ok(10),
            '9' => Ok(9),
            '8' => Ok(8),
            '7' => Ok(7),
            '6' => Ok(6),
            '5' => Ok(5),
            '4' => Ok(4),
            '3' => Ok(3),
            '2' => Ok(2),
            _ => bail!("Invalid card value"),
        }
    }
}
