mod days;
use crate::days::*;
use anyhow::Result;
use std::fs;

#[allow(dead_code, unused_macros)]
macro_rules! day {($n:literal, $p:ident) => {paste::paste!{println!("{}",[<day $n>]::$p(&String::from_utf8(fs::read(concat!("inputs/day", stringify!($n), ".txt"))?)?)?)}};}

#[allow(dead_code, unused_macros)]
macro_rules! tst {($n:literal, $p:ident) => {paste::paste!{println!("{}",[<day $n>]::$p(&String::from_utf8(fs::read(concat!("inputs/tst", stringify!($n), ".txt"))?)?)?)}};}

fn main() -> Result<()> {
    day!(03, b);
    Ok(())
}
