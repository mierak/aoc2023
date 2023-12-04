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

            for i in 0..chars.len() {
                let ele = chars[i];
                if let Some(val) = find_num_from_word_of_len(&chars, i) {
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

fn find_num_from_word_of_len(chars: &[char], idx: usize) -> Option<u32> {
    if chars.len() < idx {
        return None;
    }
    match chars[..=idx] {
        [.., 'o', 'n', 'e'] => Some(1),
        [.., 't', 'w', 'o'] => Some(2),
        [.., 's', 'i', 'x'] => Some(6),
        [.., 'f', 'o', 'u', 'r'] => Some(4),
        [.., 'f', 'i', 'v', 'e'] => Some(5),
        [.., 'n', 'i', 'n', 'e'] => Some(9),
        [.., 't', 'h', 'r', 'e', 'e'] => Some(3),
        [.., 's', 'e', 'v', 'e', 'n'] => Some(7),
        [.., 'e', 'i', 'g', 'h', 't'] => Some(8),
        _ => None,
    }
}
