use std::{
    collections::{BTreeSet, HashSet},
    mem::swap,
};

#[allow(unused_imports)]
use anyhow::{Result, anyhow};

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
    z: usize,
}

impl Pos {
    fn from_str(s: &str) -> Self {
        let mut s = s.split(',').map(|n| n.parse::<usize>().unwrap());
        let (x, y, z) = (s.next().unwrap(), s.next().unwrap(), s.next().unwrap());
        Self { x, y, z }
    }

    fn dist(&self, oth: &Self) -> usize {
        let x1 = self.x as isize;
        let y1 = self.y as isize;
        let z1 = self.z as isize;
        let x2 = oth.x as isize;
        let y2 = oth.y as isize;
        let z2 = oth.z as isize;
        ((x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2)).isqrt() as usize
    }
}

#[allow(dead_code, unused_variables)]
pub fn a(input: &str) -> Result<String> {
    let mut lines = input.lines();
    // I'll pass down how many pairs should I connect in the first line
    let n = lines.next().unwrap().parse::<usize>()?;
    let mut boxes = Vec::<Pos>::new();
    for line in lines {
        boxes.push(Pos::from_str(line));
    }
    let mut dists = BTreeSet::<(usize, usize, usize)>::new();
    for (aid, abox) in boxes.iter().enumerate() {
        for (bid, bbox) in boxes.iter().enumerate().skip(aid + 1) {
            let dist = abox.dist(bbox);
            dists.insert((dist, aid, bid));
        }
    }
    let mut circuits = Vec::<HashSet<usize>>::new();
    for (dist, aid, bid) in dists.iter().take(n) {
        let mut acid = None;
        let mut bcid = None;
        for (cid, circuit) in circuits.iter().enumerate() {
            if circuit.contains(aid) {
                acid = Some(cid);
            }
            if circuit.contains(bid) {
                bcid = Some(cid);
            }
            if acid.is_some() && bcid.is_some() {
                break;
            }
        }
        match (acid, bcid) {
            (None, None) => circuits.push(HashSet::from([*aid, *bid])),
            (None, Some(bcid)) => _ = circuits[bcid].insert(*aid),
            (Some(acid), None) => _ = circuits[acid].insert(*bid),
            // Merge two circuits (larger id to smaller for less shifting)
            (Some(acid), Some(bcid)) => {
                if acid != bcid {
                    let min_cid = acid.min(bcid);
                    let max_cid = acid.max(bcid);
                    let to_move = circuits.remove(max_cid);
                    circuits[min_cid].extend(to_move);
                }
            }
        }
    }
    let mut maxes = [0, 0, 0];
    for c in circuits.iter() {
        let mut v = c.len();
        for m in maxes.iter_mut() {
            if *m < v {
                swap(m, &mut v);
            }
        }
    }
    let sum = maxes.into_iter().reduce(|a, b| a * b).unwrap();
    Ok(format!("{}", sum))
}

#[allow(dead_code, unused_variables)]
pub fn b(input: &str) -> Result<String> {
    Ok(format!("{}", 0x70D0))
}
