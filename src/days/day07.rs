use std::collections::HashSet;

#[allow(unused_imports)]
use anyhow::{Result, anyhow};

#[allow(dead_code, unused_variables)]
pub fn a(input: &str) -> Result<String> {
    let mut beams = HashSet::<usize>::new();
    let mut sum = 0;
    for (y, line) in input.lines().enumerate() {
        let mut new_beams = HashSet::<usize>::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    new_beams.insert(x);
                }
                '.' => {
                    if beams.contains(&x) {
                        new_beams.insert(x);
                    }
                }
                '^' => {
                    if beams.contains(&x) {
                        new_beams.insert(x - 1);
                        new_beams.insert(x + 1);
                        sum += 1;
                    }
                }
                _ => unreachable!(),
            }
        }
        beams = new_beams;
    }
    Ok(format!("{}", sum))
}

#[allow(dead_code, unused_variables)]
pub fn b(input: &str) -> Result<String> {
    Ok(format!("{}", 0x70D0))
}
