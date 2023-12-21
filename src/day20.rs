use anyhow::{bail, Context, Result};
use itertools::Itertools;
use std::{
    collections::{HashMap, VecDeque},
    rc::Rc,
    str::FromStr,
};

pub fn part1(input: &str) -> Result<i64> {
    let mut modules = input.parse::<Network>()?.modules;

    for _ in 0..1000 {
        let mut queue = modules
            .get_mut("broadcaster")
            .context("no broadcaster")?
            .process(Signal::Low, "broadcaster".into());

        while let Some((dest_name, pulse, source_name)) = queue.pop_front() {
            if let Some(module) = modules.get_mut(&dest_name) {
                module
                    .process(pulse, source_name)
                    .iter()
                    .for_each(|v| queue.push_back(v.clone()));
            }
        }
    }

    let res = modules.iter().fold((0, 0), |acc, val| {
        let vals = val.1.get_counts();
        (acc.0 + vals.0, acc.1 + vals.1)
    });

    Ok((res.0 as i64 + 1000) * res.1 as i64)
}

pub fn part2(input: &str) -> Result<i64> {
    let mut modules = input.parse::<Network>()?.modules;

    let rx_input = modules
        .iter()
        .find(|(_, v)| v.outputs().contains(&"rx".into()))
        .map(|v| v.0.clone())
        .context("To find input for rc")?;
    let rx_input_parent_count = modules.iter().filter(|(_, v)| v.outputs().contains(&rx_input)).count();

    let mut result: HashMap<Rc<str>, i64> = HashMap::new();

    let mut i = 0;
    loop {
        let mut queue = modules
            .get_mut("broadcaster")
            .context("no broadcaster")?
            .process(Signal::Low, "broadcaster".into());
        i += 1;

        while let Some((dest_name, pulse, source_name)) = queue.pop_front() {
            if let Some(module) = modules.get_mut(&dest_name) {
                module
                    .process(pulse, source_name)
                    .iter()
                    .for_each(|v| queue.push_back(v.clone()));
            }
            if let Module::Conjunction(Conjuction { memory, .. }) =
                modules.get_mut(&rx_input).context("No rx parent")?
            {
                memory.iter().for_each(|v| {
                    if v.1 == &Signal::High && !result.contains_key(v.0) {
                        result.insert(v.0.clone(), i);
                    }
                });
            } else {
                bail!("not a conjunction");
            }
        }

        if result.len() == rx_input_parent_count {
            break;
        }
    }

    Ok(result.into_iter().map(|v| v.1).fold(1, lcm))
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}

#[derive(Debug, strum::EnumDiscriminants)]
enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjuction),
    Broadcast(Broadcast),
}

impl Module {
    fn process(&mut self, pulse: Signal, source_name: Rc<str>) -> VecDeque<(Rc<str>, Signal, Rc<str>)> {
        let name = self.name().clone();
        match self {
            Module::FlipFlop(m) if pulse == Signal::Low => {
                m.state.flip();
                let res: VecDeque<_> = m.outputs.iter().map(|o| (o.clone(), m.state, name.clone())).collect();
                match m.state {
                    Signal::High => m.high_sent += res.len(),
                    Signal::Low => m.low_sent += res.len(),
                };
                res
            }
            Module::Conjunction(m) => {
                let remembered_pulse = m.memory.entry(source_name.clone()).or_insert(Signal::Low);
                *remembered_pulse = pulse;
                let signal = if m.memory.iter().all(|(_, v)| *v == Signal::High) {
                    Signal::Low
                } else {
                    Signal::High
                };
                let res: VecDeque<_> = m.outputs.iter().map(|o| (o.clone(), signal, name.clone())).collect();
                match signal {
                    Signal::High => m.high_sent += res.len(),
                    Signal::Low => m.low_sent += res.len(),
                };
                res
            }
            Module::Broadcast(m) => {
                let res: VecDeque<_> = m.outputs.iter().map(|o| (o.clone(), pulse, name.clone())).collect();
                match pulse {
                    Signal::High => m.high_sent += res.len(),
                    Signal::Low => m.low_sent += res.len(),
                };
                res
            }
            Module::FlipFlop(_) => VecDeque::default(),
        }
    }

    fn init_con(&mut self, source_name: Rc<str>) {
        if let Module::Conjunction(m) = self {
            m.memory.entry(source_name).or_insert(Signal::Low);
        }
    }

    fn outputs(&self) -> &Vec<Rc<str>> {
        match self {
            Module::FlipFlop(m) => &m.outputs,
            Module::Conjunction(m) => &m.outputs,
            Module::Broadcast(m) => &m.outputs,
        }
    }

    fn name(&self) -> &Rc<str> {
        match self {
            Module::FlipFlop(m) => &m.name,
            Module::Conjunction(m) => &m.name,
            Module::Broadcast(m) => &m.name,
        }
    }

    fn get_counts(&self) -> (usize, usize) {
        match self {
            Module::FlipFlop(m) => (m.low_sent, m.high_sent),
            Module::Conjunction(m) => (m.low_sent, m.high_sent),
            Module::Broadcast(m) => (m.low_sent, m.high_sent),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Signal {
    High,
    Low,
}

impl Signal {
    fn flip(&mut self) {
        *self = match self {
            Signal::High => Signal::Low,
            Signal::Low => Signal::High,
        }
    }
}

#[derive(Debug)]
struct FlipFlop {
    name: Rc<str>,
    outputs: Vec<Rc<str>>,
    state: Signal,
    low_sent: usize,
    high_sent: usize,
}

#[derive(Debug)]
struct Conjuction {
    name: Rc<str>,
    outputs: Vec<Rc<str>>,
    memory: HashMap<Rc<str>, Signal>,
    low_sent: usize,
    high_sent: usize,
}

#[derive(Debug)]
struct Broadcast {
    name: Rc<str>,
    outputs: Vec<Rc<str>>,
    low_sent: usize,
    high_sent: usize,
}

#[derive(Debug)]
struct Network {
    modules: HashMap<Rc<str>, Module>,
}

impl FromStr for Network {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut modules: HashMap<_, _> = s
            .lines()
            .filter_map(|v| v.parse::<Module>().ok())
            .map(|m| (m.name().clone(), m))
            .collect();

        let all_modules = modules
            .values()
            .map(|m| (m.name().clone(), m.outputs().clone()))
            .collect_vec();

        for (source_name, destinations) in all_modules {
            for module in destinations {
                if let Some(m) = modules.get_mut(&module) {
                    m.init_con(Rc::clone(&source_name));
                }
            }
        }
        Ok(Network { modules })
    }
}

impl FromStr for Module {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (name, outputs) = s.split_once(" -> ").context("invalid input")?;
        let outputs = outputs.split(", ").map(Rc::from).collect_vec();
        Ok(match name {
            "broadcaster" => Module::Broadcast(Broadcast {
                outputs,
                name: Rc::from(name),
                low_sent: 0,
                high_sent: 0,
            }),
            n if n.starts_with('%') => Module::FlipFlop(FlipFlop {
                outputs,
                name: Rc::from(n.trim_start_matches('%').to_string()),
                state: Signal::Low,
                low_sent: 0,
                high_sent: 0,
            }),
            n if n.starts_with('&') => Module::Conjunction(Conjuction {
                outputs,
                name: Rc::from(n.trim_start_matches('&').to_string()),
                memory: HashMap::default(),
                low_sent: 0,
                high_sent: 0,
            }),
            _ => bail!("invalid input"),
        })
    }
}
