use anyhow::{bail, Result};

pub fn part1(input: &str) -> Result<String> {
    Ok(input
        .lines()
        .map(|val| -> Result<i32> {
            let mut res = Res::default();
            for ele in val.chars() {
                if ele.is_numeric() {
                    let Some(ele) = ele.to_digit(10) else {
                        bail!("Cannot unwrap numeric char")
                    };
                    res.set(ele);
                }
            }
            res.to_number()
        })
        .sum::<Result<i32>>()?
        .to_string())
}

pub fn part2(input: &str) -> Result<String> {
    Ok(input
        .lines()
        .map(|val| -> Result<i32> {
            let mut res = Res::default();
            let chars: Vec<char> = val.chars().collect();

            for i in 0..val.chars().count() {
                let ele = chars[i];
                if let Some(val) = find_num_from_word_of_len(&chars, 5, i) {
                    res.set(val);
                } else if let Some(val) = find_num_from_word_of_len(&chars, 4, i) {
                    res.set(val);
                } else if let Some(val) = find_num_from_word_of_len(&chars, 3, i) {
                    res.set(val);
                } else if ele.is_numeric() {
                    let Some(ele) = ele.to_digit(10) else {
                        bail!("Cannot unwrap numeric char")
                    };
                    res.set(ele);
                }
            }

            res.to_number()
        })
        .sum::<Result<i32>>()?
        .to_string())
}

#[derive(Default, Debug)]
struct Res {
    first: Option<u32>,
    last: Option<u32>,
}

impl Res {
    fn set(&mut self, val: u32) {
        self.try_set_first(val);
        self.last = Some(val);
    }
    fn try_set_first(&mut self, val: u32) {
        if self.first.is_none() {
            self.first = Some(val);
        }
    }

    fn to_number(&self) -> Result<i32> {
        let Some(first) = self.first else {
            bail!("Cannot finalize result. '{:?}'", self)
        };
        let Some(last) = self.last else {
            bail!("Cannot finalize result. '{:?}'", self)
        };
        Ok(format!("{}{}", first, last).parse::<i32>()?)
    }
}

fn find_num_from_word_of_len(chars: &[char], len: usize, idx: usize) -> Option<u32> {
    if idx >= (len - 1) {
        let word = String::from_iter(&chars[idx - (len - 1)..=idx]);
        if word.ends_with("one") {
            Some(1)
        } else if word.ends_with("two") {
            Some(2)
        } else if word.ends_with("three") {
            Some(3)
        } else if word.ends_with("four") {
            Some(4)
        } else if word.ends_with("five") {
            Some(5)
        } else if word.ends_with("six") {
            Some(6)
        } else if word.ends_with("seven") {
            Some(7)
        } else if word.ends_with("eight") {
            Some(8)
        } else if word.ends_with("nine") {
            Some(9)
        } else {
            None
        }
    } else {
        None
    }
}
