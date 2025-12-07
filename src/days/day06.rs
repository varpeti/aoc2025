#[allow(unused_imports)]
use anyhow::{Result, anyhow};

#[derive(Debug)]
enum Op {
    Add(u64),
    Mul(u64),
}
impl Op {
    fn op(&mut self, num: u64) {
        match self {
            Op::Add(value) => *value += num,
            Op::Mul(value) => *value *= num,
        }
    }

    fn get(&self) -> u64 {
        match self {
            Op::Add(value) => *value,
            Op::Mul(value) => *value,
        }
    }
}

#[allow(dead_code, unused_variables)]
pub fn a(input: &str) -> Result<String> {
    let mut table = Vec::<Vec<u64>>::new();
    let mut ops = Vec::<Op>::new();
    for line in input.lines() {
        let mut tline = Vec::<u64>::new();
        for token in line.split_whitespace() {
            match token {
                "+" => ops.push(Op::Add(0)),
                "*" => ops.push(Op::Mul(1)),
                num => tline.push(num.parse()?),
            };
        }
        if !tline.is_empty() {
            table.push(tline);
        }
    }
    #[allow(clippy::needless_range_loop)]
    for i in 0..table[0].len() {
        for j in 0..table.len() {
            ops[i].op(table[j][i]);
        }
    }
    let sum = ops
        .iter()
        .map(|op| op.get())
        .reduce(|acc, e| acc + e)
        .unwrap();
    Ok(format!("{}", sum))
}

#[allow(dead_code, unused_variables)]
pub fn b(input: &str) -> Result<String> {
    Ok(format!("{}", 0x70D0))
}
