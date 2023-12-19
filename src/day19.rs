use anyhow::{bail, Context, Result};
use itertools::Itertools;
use std::{collections::HashMap, str::FromStr};

pub fn part1(input: &str) -> Result<i64> {
    let (workflows, parts) = input.split_once("\n\n").context("Invalid input format")?;
    let workflows: HashMap<String, Vec<Rule>> = workflows
        .lines()
        .map(|w| -> Result<_> {
            let w = w.parse::<Workflow>()?;
            Ok((w.name, w.rules))
        })
        .try_collect()?;
    let parts: Vec<Part> = parts.lines().map(|p| p.parse()).try_collect()?;

    let sum = parts
        .iter()
        .filter(|part| {
            let mut rule_name = "in";
            loop {
                let Some(rules) = workflows.get(rule_name) else {
                    return false;
                };
                for rule in rules {
                    if part.matches(rule) {
                        match &rule.action {
                            Action::Accept => return true,
                            Action::Reject => return false,
                            Action::Send(dest) => {
                                rule_name = dest;
                                break;
                            }
                        }
                    }
                }
            }
        })
        .map(Part::sum)
        .sum::<i64>();

    Ok(sum)
}

pub fn part2(_input: &str) -> Result<i64> {
    Ok(0)
}

#[derive(Debug)]
struct Part {
    cool: i64,
    musical: i64,
    aerodynamic: i64,
    shiny: i64,
}

impl Part {
    fn value(&self, category: &Category) -> i64 {
        match category {
            Category::ExtremelyCool => self.cool,
            Category::Musical => self.musical,
            Category::Aerodynamic => self.aerodynamic,
            Category::Shiny => self.shiny,
        }
    }

    fn sum(&self) -> i64 {
        self.cool + self.musical + self.aerodynamic + self.shiny
    }

    fn matches(&self, rule: &Rule) -> bool {
        if let Some(ref condition) = rule.condition {
            match condition.condition_type {
                ConditionType::LessThan => condition.value > self.value(&condition.category),
                ConditionType::MoreThan => condition.value < self.value(&condition.category),
            }
        } else {
            true
        }
    }
}

impl FromStr for Part {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut categories = s.trim_start_matches('{').trim_end_matches('}').split(',');
        let (_, cool) = categories
            .next()
            .context("Invalid part")?
            .split_once('=')
            .context("Invalid part")?;
        let (_, musical) = categories
            .next()
            .context("Invalid part")?
            .split_once('=')
            .context("Invalid part")?;
        let (_, aerodynamic) = categories
            .next()
            .context("Invalid part")?
            .split_once('=')
            .context("Invalid part")?;
        let (_, shiny) = categories
            .next()
            .context("Invalid part")?
            .split_once('=')
            .context("Invalid part")?;

        Ok(Part {
            cool: cool.parse()?,
            musical: musical.parse()?,
            aerodynamic: aerodynamic.parse()?,
            shiny: shiny.parse()?,
        })
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl FromStr for Workflow {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rest) = s.split_once('{').context("Invalid wokrflow format")?;
        let rules = rest
            .trim_matches('}')
            .split(',')
            .map(|w| w.parse::<Rule>())
            .try_collect()?;
        Ok(Workflow {
            name: name.to_string(),
            rules,
        })
    }
}

#[derive(Debug)]
struct Rule {
    condition: Option<Condition>,
    action: Action,
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((condition, action)) = s.split_once(':') else {
            return Ok(Rule {
                condition: None,
                action: s.parse().context("Invalid default action")?,
            });
        };
        let action: Action = action.parse().context("Invalid action format")?;
        let mut chars = condition.chars();
        let category: Category = chars.next().context("Invalid category format")?.try_into()?;
        let condition_type: ConditionType = chars.next().context("Invalid condition format")?.try_into()?;
        let value = chars.collect::<String>().parse::<i64>().context("Invalid rule value")?;
        Ok(Rule {
            condition: Some(Condition {
                category,
                condition_type,
                value,
            }),
            action,
        })
    }
}

#[derive(Debug)]
enum Category {
    ExtremelyCool,
    Musical,
    Aerodynamic,
    Shiny,
}

impl TryFrom<char> for Category {
    type Error = anyhow::Error;

    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            'x' => Ok(Category::ExtremelyCool),
            'm' => Ok(Category::Musical),
            'a' => Ok(Category::Aerodynamic),
            's' => Ok(Category::Shiny),
            _ => bail!("Invalid category"),
        }
    }
}

#[derive(Debug)]
struct Condition {
    condition_type: ConditionType,
    value: i64,
    category: Category,
}

impl TryFrom<char> for ConditionType {
    type Error = anyhow::Error;

    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            '>' => Ok(ConditionType::MoreThan),
            '<' => Ok(ConditionType::LessThan),
            _ => bail!("Invalid condition type"),
        }
    }
}

#[derive(Debug)]
enum ConditionType {
    LessThan,
    MoreThan,
}

#[derive(Debug)]
enum Action {
    Accept,
    Reject,
    Send(String),
}

impl FromStr for Action {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Action::Accept,
            "R" => Action::Reject,
            v => Action::Send(v.to_string()),
        })
    }
}
