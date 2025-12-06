#[allow(unused_imports)]
use anyhow::{Result, anyhow};

#[allow(dead_code, unused_variables)]
pub fn a(input: &str) -> Result<String> {
    let mut sum = 0;
    for bank in input.lines() {
        let mut bank = bank.chars();
        let last = bank.nth_back(0).unwrap();
        let mut max = ['0'; 2];
        for c in bank {
            if c > max[0] {
                max[0] = c;
                max[1] = '0';
                continue;
            }
            if c > max[1] {
                max[1] = c;
                continue;
            }
        }
        if last > max[1] {
            max[1] = last;
        }
        sum += max[0].to_digit(10).unwrap() * 10 + max[1].to_digit(10).unwrap();
    }
    Ok(format!("{}", sum))
}

#[allow(dead_code, unused_variables)]
pub fn b(input: &str) -> Result<String> {
    let mut sum = 0u64;
    for bank in input.lines() {
        let mut s = 0usize;
        let mut e = bank.len() - 11;
        let mut max = ['0'; 12];
        for m in max.iter_mut() {
            let mut next_s = s + 1;
            for i in s..e {
                let c = bank.as_bytes()[i] as char;
                if c > *m {
                    *m = c;
                    next_s = i + 1;
                }
            }
            s = next_s;
            e += 1;
        }
        sum += max.into_iter().collect::<String>().parse::<u64>().unwrap();
    }
    Ok(format!("{}", sum))
}
