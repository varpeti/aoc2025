use std::collections::BTreeMap;

#[allow(unused_imports)]
use anyhow::{Result, anyhow};
use pathfinding::prelude::count_paths;

type Nodes<'a> = BTreeMap<&'a str, Vec<&'a str>>;

#[allow(dead_code, unused_variables)]
pub fn a(input: &str) -> Result<String> {
    let mut nodes = Nodes::new();
    for line in input.lines() {
        let (from, to) = line.split_once(':').unwrap();
        let to = to.split_whitespace().collect::<Vec<&str>>();
        nodes.insert(from, to);
    }
    let sum = count_paths("you", |&pos| nodes[pos].clone(), |&pos| pos == "out");
    Ok(format!("{}", sum))
}

#[allow(dead_code, unused_variables)]
pub fn b(input: &str) -> Result<String> {
    let mut nodes = Nodes::new();
    for line in input.lines() {
        let (from, to) = line.split_once(':').unwrap();
        let to = to.split_whitespace().collect::<Vec<&str>>();
        nodes.insert(from, to);
    }
    nodes.insert("out", vec![]);
    let dac_fft = count_paths("dac", |&pos| nodes[pos].clone(), |&pos| pos == "fft");
    let sum = if dac_fft != 0 {
        let svr_dac = count_paths("svr", |&pos| nodes[pos].clone(), |&pos| pos == "dac");
        let fft_out = count_paths("fft", |&pos| nodes[pos].clone(), |&pos| pos == "out");
        svr_dac * dac_fft * fft_out
    } else {
        let fft_dac = count_paths("fft", |&pos| nodes[pos].clone(), |&pos| pos == "dac");
        let svr_fft = count_paths("svr", |&pos| nodes[pos].clone(), |&pos| pos == "fft");
        let dac_out = count_paths("dac", |&pos| nodes[pos].clone(), |&pos| pos == "out");
        svr_fft * fft_dac * dac_out
    };
    Ok(format!("{}", sum))
}
