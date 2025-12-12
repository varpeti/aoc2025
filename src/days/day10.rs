#[allow(unused_imports)]
use anyhow::{Result, anyhow};
use pathfinding::{prelude::astar, prelude::dijkstra};

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
    // So it probaly solves it...
    // But it will take like forever
    // So no ☀️ for me (yet) 😭
    // // Probably there is an advanced linear algebraic solution
    // But I have not enough knowlege (yet) and don't want to rely on AI
    // Also there is many optimalization which could be done (Vec of bools to int, etc)
    let mut sum = 0;
    for line in input.lines() {
        let machine = Machine::from_str(line)?;
        let (_, min) = astar(
            &vec![0; machine.joltage.len()],
            |pos: &Vec<usize>| {
                let mut successors = Vec::with_capacity(machine.buttons.len());
                for button in machine.buttons.iter() {
                    let mut new_pos = pos.clone();
                    let mut ok = true;
                    for b in button {
                        new_pos[*b] += 1;
                        if new_pos[*b] > machine.joltage[*b] {
                            ok = false;
                            break;
                        }
                    }
                    if ok {
                        // Probably there is a better cost
                        successors.push((new_pos, 1));
                    }
                }
                successors
            },
            |pos| {
                // Probably there is a better heuristic
                let mut h = 0;
                for (i, j) in pos.iter().enumerate() {
                    h += machine.joltage[i] - j;
                }
                h
            },
            |pos| *pos == machine.joltage,
        )
        .ok_or_else(|| anyhow!("Expected solution!"))?;
        sum += min;
    }
    Ok(format!("{}", sum))
}
