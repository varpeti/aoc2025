#[allow(unused_imports)]
use anyhow::{Result, anyhow};
use fancy_regex::Regex;

#[allow(dead_code, unused_variables)]
pub fn a(input: &str) -> Result<String> {
    let repeat_reg = Regex::new(r#"^(\d+)\1$"#).expect("Valid regex");
    let ranges = input.trim().split(',');
    let mut sum = 0u64;
    for range in ranges {
        let (s, e) = range
            .split_once('-')
            .ok_or_else(|| anyhow!("Expected - in `{}`", range))?;
        let s = s
            .parse::<usize>()
            .map_err(|err| anyhow!("{} Not a number? `{}`", err, s))?;
        let e = e
            .parse::<usize>()
            .map_err(|err| anyhow!("{} Not a number? `{}`", err, e))?;
        for num in s..=e {
            if repeat_reg.is_match(&format!("{}", num))? {
                sum += num as u64;
            }
        }
    }
    Ok(format!("{}", sum))
}

#[allow(dead_code, unused_variables)]
pub fn b(input: &str) -> Result<String> {
    let repeat_reg = Regex::new(r#"^(\d+)(?:\1)+$"#).expect("Valid regex");
    let ranges = input.trim().split(',');
    let mut sum = 0u64;
    for range in ranges {
        let (s, e) = range
            .split_once('-')
            .ok_or_else(|| anyhow!("Expected - in `{}`", range))?;
        let s = s
            .parse::<usize>()
            .map_err(|err| anyhow!("{} Not a number? `{}`", err, s))?;
        let e = e
            .parse::<usize>()
            .map_err(|err| anyhow!("{} Not a number? `{}`", err, e))?;
        for num in s..=e {
            if repeat_reg.is_match(&format!("{}", num))? {
                sum += num as u64;
            }
        }
    }
    Ok(format!("{}", sum))
}
