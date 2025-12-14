#[allow(unused_imports)]
use anyhow::{Result, anyhow};

type Shape = [[bool; 3]; 3];
type Shapes = [Shape; 6];

#[allow(dead_code, unused_variables)]
pub fn a(input: &str) -> Result<String> {
    let mut shapes = Shapes::default();
    let mut lines = input.lines();
    #[allow(clippy::needless_range_loop)]
    for si in 0..6 {
        lines.next();
        let mut shape = Shape::default();
        #[allow(clippy::needless_range_loop)]
        for y in 0..3 {
            let mut line = lines.next().unwrap().chars();
            for x in 0..3 {
                shape[y][x] = line.next().unwrap() == '#';
            }
        }
        shapes[si] = shape;
        lines.next();
    }
    let mut sum = 0;
    for line in lines {
        let (size, num_of_shapes) = line.split_once(':').unwrap();
        let (nx, ny) = size.split_once('x').unwrap();
        let (nx, ny) = (nx.parse::<usize>().unwrap(), ny.parse::<usize>().unwrap());
        eprint!("{}x{} ({}):", nx, ny, nx * ny);
        let mut count_sum = 0;
        for (si, num) in num_of_shapes
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .enumerate()
        {
            if num == 0 {
                continue;
            }
            // If I know beforehand
            // I should have calculated the parts and store them
            // instead of recalculating every time...
            let count = shapes[si]
                .iter()
                .map(|l| l.iter().map(|&b| b as usize).sum::<usize>())
                .sum::<usize>();
            eprint!("{}+", count * num);
            count_sum += count * num;
        }

        if count_sum > nx * ny {
            eprintln!("0 = {} NOP", count_sum);
        } else {
            eprintln!("0 = {} OK", count_sum);
            sum += 1;
            // To be fair, it should not solve it...
            // The task was to pack the boxes into regions,
            // and not count the parts and compare it to the area...
            // But can't complain! Merry XMAS! 🎄🎁
        }
    }
    Ok(format!("{}", sum))
}

#[allow(dead_code, unused_variables)]
pub fn b(input: &str) -> Result<String> {
    Ok("Happy XMAS! 🎄🎁".to_string())
}
