#[allow(unused_imports)]
use anyhow::{Result, anyhow};

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Paper,
}
impl Cell {
    fn from_char(c: char) -> Result<Self> {
        Ok(match c {
            '.' => Cell::Empty,
            '@' => Cell::Paper,
            oth => return Err(anyhow!("Expected '.' or '@' but got `{}`", oth)),
        })
    }
}

type Room = Vec<Vec<Cell>>;

fn get_neighbours(room: &Room, y: usize, x: usize) -> Vec<&Cell> {
    let mut neighbours = Vec::new();
    for iy in -1..=1 {
        for ix in -1..=1 {
            if iy == 0 && ix == 0 {
                continue;
            }
            let y = y as isize + iy;
            if y < 0 || y >= room.len() as isize {
                continue;
            }
            let x = x as isize + ix;
            if x < 0 || x >= room[0].len() as isize {
                continue;
            }
            neighbours.push(&room[y as usize][x as usize])
        }
    }
    neighbours
}

#[allow(dead_code, unused_variables)]
pub fn a(input: &str) -> Result<String> {
    let room: Room = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| Cell::from_char(c).ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;

    for y in 0..room.len() {
        for x in 0..room[0].len() {
            if room[y][x] == Cell::Paper
                && get_neighbours(&room, y, x)
                    .into_iter()
                    .filter(|c| **c == Cell::Paper)
                    .count()
                    < 4
            {
                sum += 1;
            }
        }
    }

    Ok(format!("{}", sum))
}

#[allow(dead_code, unused_variables)]
pub fn b(input: &str) -> Result<String> {
    let mut room: Room = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| Cell::from_char(c).ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    loop {
        let mut end = true;
        for y in 0..room.len() {
            for x in 0..room[0].len() {
                if room[y][x] == Cell::Paper
                    && get_neighbours(&room, y, x)
                        .into_iter()
                        .filter(|c| **c == Cell::Paper)
                        .count()
                        < 4
                {
                    room[y][x] = Cell::Empty;
                    sum += 1;
                    end = false;
                }
            }
        }
        if end {
            break;
        }
    }

    Ok(format!("{}", sum))
}
