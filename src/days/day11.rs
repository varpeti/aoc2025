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
    Ok(format!("{}", 0x70D0))
}
