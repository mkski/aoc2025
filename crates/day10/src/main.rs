use aoc2025::utils;
use itertools::Itertools;
use regex::Regex;
use std::collections::VecDeque;
use std::{cmp::min, env, usize};
use z3::Optimize;
use z3::ast::Int;

type Button = Vec<usize>;

#[derive(Debug, Clone, Copy)]
struct Solution {
    #[allow(unused)]
    part1: usize,
    #[allow(unused)]
    part2: usize,
}

#[derive(Debug, Clone)]
struct Machine {
    indicators: Vec<bool>,
    target: Vec<bool>,
    buttons: Vec<Button>,
    joltages: Vec<usize>,
}

impl Machine {
    pub fn power_on(&mut self) -> Vec<&Button> {
        let mut queue: VecDeque<Vec<&Button>> =
            VecDeque::from_iter(self.buttons.iter().map(|b| vec![b]));
        while let Some(sequence) = queue.pop_front() {
            let turned_on = sequence
                .iter()
                .fold(self.indicators.clone(), |state, &seq| {
                    let mut new_state = state.clone();
                    for b in seq {
                        new_state[*b as usize] = !state[*b as usize];
                    }
                    new_state
                });
            let success = turned_on
                .iter()
                .enumerate()
                .map(|(idx, i)| self.target[idx] == *i)
                .reduce(|acc, i| acc && i)
                .unwrap();

            if success {
                return sequence;
            }

            for button in self.buttons.iter() {
                let mut new_sequence = sequence.clone();
                new_sequence.push(button);
                queue.push_back(new_sequence);
            }
        }
        Vec::new()
    }

pub fn configure_joltages(&self) -> Option<usize> {
    let opt = Optimize::new();
    let mut buttons: Vec<Int> = Vec::new();
    for (i, _) in self.buttons.iter().enumerate() {
        let bi = Int::new_const(format!("B{i}").as_str());
        opt.assert(&bi.ge(0));
        buttons.push(bi);
    }

    for (idx, joltage) in self.joltages.iter().enumerate() {
        let can_increase = self
            .buttons
            .iter()
            .enumerate()
            .filter_map(|(bidx, button)| {
                if button.contains(&idx) {
                    Some(buttons[bidx].clone())
                } else {
                    None
                }
            });
        let sum = can_increase.fold(Int::from(0u64), |eq, b| eq + b);
        opt.assert(&sum.eq(*joltage as u64));
    }
    let opt_eq = buttons.iter().fold(Int::from(0), |eq, b| eq + b);
    opt.minimize(&opt_eq);

    if let z3::SatResult::Sat = opt.check(&[]) {
        let model = opt.get_model().unwrap();
        let o = model.eval(&opt_eq, true).unwrap();
        Some(o.as_u64().unwrap() as usize)
    } else {
        None
    }
}
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = utils::read_input(&filename);
    let input_re = Regex::new(
        r"(?P<indicators>\[[\.#]+\]) (?P<buttons>(\((\d,?)+\) )+)(?P<joltages>\{(\d,?)+\})",
    )
    .unwrap();

    let machines = input_re.captures_iter(&input).map(|cap| {
        let target = cap["indicators"]
            .trim_matches(['[', ']'])
            .chars()
            .map(|c| (c == '#'))
            .collect::<Vec<bool>>();
        let mut indicators = Vec::new();
        for _ in 0..target.len() {
            indicators.push(false);
        }
        let buttons = cap["buttons"]
            .trim()
            .split(' ')
            .map(|button| {
                button
                    .trim_matches(['(', ')'])
                    .split(',')
                    .map(|i| i.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();
        let joltages = cap["joltages"]
            .trim_matches(['{', '}'])
            .split(',')
            .map(|i| i.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        Machine {
            indicators,
            target,
            buttons,
            joltages,
        }
    }).collect::<Vec<Machine>>();

    let solution = Solution {
        part1: machines.clone().iter_mut().map(|m| m.power_on().len()).sum(),
        part2: machines
            .iter()
            .filter_map(|m| m.configure_joltages())
            .sum(),
    };
    println!("{solution:?}")
}
