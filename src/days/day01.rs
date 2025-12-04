use anyhow::{Result, anyhow};

#[allow(dead_code, unused_variables)]
pub fn a(input: &str) -> Result<String> {
    let mut pos = 50;
    let mut pw = 0;
    for line in input.lines() {
        let (dir, num) = line.split_at(1);
        let num = num.parse::<i32>()?;
        pos = match dir {
            "L" => i32::rem_euclid(pos - num, 100),
            "R" => i32::rem_euclid(pos + num, 100),
            _ => Err(anyhow!("Expected L or R"))?,
        };
        if pos == 0 {
            pw += 1;
        }
    }
    Ok(format!("{}", pw))
}

#[allow(dead_code, unused_variables)]
pub fn b(input: &str) -> Result<String> {
    let mut pos = 50;
    let mut pw = 0;
    for line in input.lines() {
        let (dir, num) = line.split_at(1);
        let num = num.parse::<i32>()?;
        let sig = match dir {
            "L" => -1,
            "R" => 1,
            _ => Err(anyhow!("Expected L or R"))?,
        };
        // TODO: do math istead of simulation...
        for _ in 0..num {
            pos = i32::rem_euclid(pos + sig, 100);
            if pos == 0 {
                pw += 1;
            }
        }
    }
    Ok(format!("{}", pw))
}
