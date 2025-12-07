use std::ops::RangeInclusive;

#[allow(unused_imports)]
use anyhow::{Result, anyhow};

#[allow(dead_code, unused_variables)]
pub fn a(input: &str) -> Result<String> {
    let mut ranges = Vec::new();
    let mut sum = 0u32;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        match line.split_once('-') {
            Some((s, e)) => {
                let s = s.parse::<u64>()?;
                let e = e.parse::<u64>()?;
                ranges.push(s..=e);
            }
            None => {
                let id = line.parse::<u64>()?;
                for range in ranges.iter() {
                    if range.contains(&id) {
                        sum += 1;
                        break;
                    }
                }
            }
        }
    }

    Ok(format!("{}", sum))
}

#[allow(dead_code, unused_variables)]
pub fn b(input: &str) -> Result<String> {
    let mut ranges = Vec::<RangeInclusive<u64>>::new();
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let (s, e) = line.split_once('-').unwrap();
        let mut s = s.parse::<u64>()?;
        let mut e = e.parse::<u64>()?;
        ranges.retain(|range| {
            if s <= *range.end() && *range.start() <= e {
                s = u64::min(s, *range.start());
                e = u64::max(e, *range.end());
                false
            } else {
                true
            }
        });
        ranges.push(s..=e);
    }

    let mut sum = 0u64;
    for range in ranges {
        sum += range.end() - range.start() + 1;
    }
    Ok(format!("{}", sum))
}
