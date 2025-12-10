use std::ops::Add;

#[allow(unused_imports)]
use anyhow::{Result, anyhow};

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn from_str(s: &str) -> Result<Self> {
        let (x, y) = s.split_once(',').ok_or_else(|| anyhow!("Expected 'x,y'"))?;
        let (x, y) = (x.parse::<usize>()?, y.parse::<usize>()?);
        Ok(Self { x, y })
    }
    fn area(&self, oth: &Pos) -> usize {
        self.x.abs_diff(oth.x).add(1) * self.y.abs_diff(oth.y).add(1)
    }
}

#[allow(dead_code, unused_variables)]
pub fn a(input: &str) -> Result<String> {
    let mut boxes = Vec::<Pos>::new();
    for line in input.lines() {
        boxes.push(Pos::from_str(line)?);
    }
    let mut max_area = 0;
    for (aid, abox) in boxes.iter().enumerate() {
        for (bid, bbox) in boxes.iter().skip(aid + 1).enumerate() {
            let area = abox.area(bbox);
            if max_area < area {
                max_area = area;
            }
        }
    }
    Ok(format!("{}", max_area))
}

#[allow(dead_code, unused_variables)]
pub fn b(input: &str) -> Result<String> {
    Ok(format!("{}", 0x70D0))
}
