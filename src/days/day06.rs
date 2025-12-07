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
    let mut table = Vec::<Vec<char>>::new();
    let mut ops = Vec::<Op>::new();
    for line in input.lines() {
        let mut tline = Vec::<char>::new();
        for token in line.chars() {
            match token {
                '+' => ops.push(Op::Add(0)),
                '*' => ops.push(Op::Mul(1)),
                num => tline.push(num),
            };
        }
        table.push(tline);
    }
    table.pop();
    let table = transpose(table);
    let mut i = 0;
    for line in table {
        let num = line.iter().collect::<String>();
        let num = num.trim();
        if num.is_empty() {
            i += 1;
            continue;
        }
        ops[i].op(num.parse()?);
    }
    let sum = ops
        .iter()
        .map(|op| op.get())
        .reduce(|acc, e| acc + e)
        .unwrap();
    Ok(format!("{}", sum))
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
