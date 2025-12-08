use aoc2025::utils;
use regex::Regex;
use std::cmp::{max, min};
use std::env;

#[derive(Debug, Clone, Copy)]
struct Solution {
    part1: usize,
    part2: usize,
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = utils::read_input(&filename);
    let col_separator = Regex::new(r"\s+").unwrap();
    let mut solution = Solution { part1: 0, part2: 0 };
    let mut grid: Vec<Vec<String>> = Vec::new();

    for line in input.lines() {
        let mut columns: Vec<String> = Vec::new();
        for col in col_separator.split(line) {
            if col.is_empty() {
                continue;
            }
            columns.push(col.to_string());
        }
        grid.push(columns);
    }

    let mut columns: Vec<Vec<String>> = Vec::new();
    let operators = grid.pop().unwrap();
    for (i, operator) in operators.iter().enumerate() {
        let mut column: Vec<String> = Vec::new();
        solution.part1 += grid.iter().fold(0, |acc, item| {
            column.push(item[i].clone());
            let num: usize = item[i].parse().unwrap();
            if acc == 0 {
                num
            } else {
                if operator == "*" {
                    acc * num
                } else {
                    acc + num
                }
            }
        });
        columns.push(column);
    }

    let mut column_lengths: Vec<usize> = Vec::new();
    for column in columns {
        column_lengths.push(column.iter().fold(0, |acc, c| max(acc, c.len())));
    }

    let mut grid: Vec<Vec<String>> = Vec::new();
    for line in input.lines() {
        let mut current = 0;
        let mut row: Vec<String> = Vec::new();
        for length in &column_lengths {
            let col = line[current..min(line.len(), current + length)].to_string();
            row.push(col);
            current += length + 1;
        }
        grid.push(row);
    }
    _ = grid.pop();

    for (i, operator) in operators.iter().enumerate() {
        let operator = if operator == "*" {
            |n1: usize, n2: usize| n1 * n2
        } else {
            |n1: usize, n2: usize| n1 + n2
        };
        let mut column: Vec<String> = Vec::new();
        for row in grid.iter() {
            column.push(row[i].clone());
        }

        let mut numbers: Vec<usize> = Vec::new();
        for l in (0..column_lengths[i]).rev() {
            let mut number: String = String::new();
            for n in column.iter() {
                if let Some(c) = n.chars().nth(l)
                    && c != ' '
                {
                    number.push(c);
                }
            }
            numbers.push(number.parse::<usize>().unwrap());
        }
        solution.part2 += numbers.iter().cloned().reduce(operator).unwrap();
    }
    println!("{:?}", solution);
}
