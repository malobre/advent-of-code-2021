/// Day 11
/// Given a grid of 100 bioluminescent octopuses, compute the number of flashes after 100 steps.
use std::io::{self, BufRead};

use anyhow::Ok;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

/// Return an iterator over the positions of adjacent octopuses.
fn neighbors_of(center @ (x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let xs = x.saturating_sub(1)..x.saturating_add(2).min(WIDTH);
    let ys = y.saturating_sub(1)..y.saturating_add(2).min(HEIGHT);

    xs.flat_map(move |x| ys.clone().map(move |y| (x, y)))
        .filter(move |&cell| cell != center)
}

/// Visit an octopus, tickling its insides.
/// Returns the number of flashes triggered.
fn visit(grid: &mut [[u8; 10]; 10], cell @ (x, y): (usize, usize)) -> u32 {
    if grid[y][x] == u8::MAX {
        return 0;
    }

    grid[y][x] += 1;

    if grid[y][x] > 9 {
        grid[y][x] = u8::MAX;
        let mut flashes = 1;

        for neighbor in neighbors_of(cell) {
            flashes += visit(grid, neighbor);
        }

        flashes
    } else {
        0
    }
}

fn main() -> Result<(), anyhow::Error> {
    let mut grid = io::stdin()
        .lock()
        .lines()
        .try_fold(([[0_u8; WIDTH]; HEIGHT], 0), |(mut grid, y), line| {
            grid[y] = line?
                .chars()
                .try_fold(([0_u8; WIDTH], 0), |(mut line, x), char| {
                    line[x] = u8::try_from(
                        char.to_digit(10)
                            .ok_or_else(|| anyhow::anyhow!("unknown digit `{}`", char))?,
                    )?;

                    Ok((line, x + 1))
                })
                .map(|(line, _)| line)?;

            Ok((grid, y + 1))
        })
        .map(|(grid, _)| grid)?;

    let mut step = 0;
    let mut flashes = Some(0);

    loop {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let n = visit(&mut grid, (x, y));

                if let Some(ref mut flashes) = flashes {
                    *flashes += n;
                }
            }
        }

        for cell in grid.iter_mut().flatten() {
            if *cell == u8::MAX {
                *cell = 0;
            }
        }

        step += 1;

        if step == 100 {
            println!("Number of flashes after 100 steps: {:?}", flashes);
            flashes = None;
        }

        if grid.iter().flatten().all(|&n| n == 0) {
            break;
        }
    }

    println!("First synchronized flash after {} step", step);

    Ok(())
}
