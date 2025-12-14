#[allow(unused_imports)]
use anyhow::{Result, anyhow};
use good_lp::{Expression, ProblemVariables, Solution, SolverModel, default_solver, variable};
use pathfinding::prelude::dijkstra;
use std::ops::AddAssign;

#[derive(Debug)]
struct Machine {
    output: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}
impl Machine {
    fn from_str(s: &str) -> Result<Self> {
        let mut sw = s.split_whitespace();
        let output_raw = sw
            .next()
            .ok_or_else(|| anyhow!("Expected '[.#]' but got {}", s))?
            .trim_matches(['[', ']']);
        let mut buttons_raw = Vec::<&str>::new();
        let mut joltage_raw = "";
        for e in sw {
            if e.starts_with('(') {
                buttons_raw.push(e.trim_matches(['(', ')']));
                continue;
            }
            joltage_raw = e.trim_matches(['{', '}']);
            break;
        }
        let mut output = Vec::new();
        for c in output_raw.chars() {
            match c {
                '.' => output.push(false),
                '#' => output.push(true),
                oth => return Err(anyhow!("Expected '.' or '#' but got {}", oth)),
            }
        }
        let mut buttons = Vec::new();
        for button_raw in buttons_raw {
            let mut button = Vec::new();
            for num in button_raw.split(',') {
                button.push(num.parse::<usize>()?);
            }
            buttons.push(button);
        }
        let mut joltage = Vec::new();
        for num in joltage_raw.split(',') {
            joltage.push(num.parse::<usize>()?);
        }
        Ok(Self {
            output,
            buttons,
            joltage,
        })
    }
}

#[allow(dead_code, unused_variables)]
pub fn a(input: &str) -> Result<String> {
    let mut sum = 0;
    for line in input.lines() {
        let machine = Machine::from_str(line)?;
        let (_, min) = dijkstra(
            &vec![false; machine.output.len()],
            |pos: &Vec<bool>| {
                let mut successors = Vec::with_capacity(machine.buttons.len());
                for button in machine.buttons.iter() {
                    let mut new_pos = pos.clone();
                    for b in button {
                        new_pos[*b] = !new_pos[*b];
                    }
                    successors.push((new_pos, 1));
                }
                successors
            },
            |pos| *pos == machine.output,
        )
        .ok_or_else(|| anyhow!("Expected solution!"))?;
        sum += min;
    }
    Ok(format!("{}", sum))
}

#[allow(dead_code, unused_variables)]
pub fn b(input: &str) -> Result<String> {
    let mut sum = 0;
    for line in input.lines() {
        let machine = Machine::from_str(line)?;
        let mut problem_variables = ProblemVariables::new();
        let mut button_variables = Vec::with_capacity(machine.buttons.len());
        for _ in 0..machine.buttons.len() {
            button_variables.push(problem_variables.add(variable().min(0).integer()));
        }
        let mut problem = problem_variables
            .minimise(button_variables.iter().sum::<Expression>())
            .using(default_solver);
        for (ji, jolt) in machine.joltage.iter().enumerate() {
            let mut expression = Expression::with_capacity(machine.buttons.len());
            for (bi, button) in machine.buttons.iter().enumerate() {
                if button.contains(&ji) {
                    expression.add_assign(button_variables[bi]);
                }
            }
            problem = problem.with(expression.eq(*jolt as u32));
        }
        let solution = problem.solve().unwrap();
        sum += button_variables
            .into_iter()
            .map(|variable| solution.value(variable))
            .sum::<f64>()
            .round() as usize;
    }

    Ok(format!("{}", sum))
}
