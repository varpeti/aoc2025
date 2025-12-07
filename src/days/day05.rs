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
    Ok(format!("{}", 0x70D0))
}
