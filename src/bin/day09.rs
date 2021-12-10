/// Day 9
/// Find the lowpoints in a heightmap and compute the size of bassins.
use std::{
    collections::HashSet,
    io::{self, BufRead},
};

use anyhow::Ok;

// Return the neighbors (above, under, left, right).
fn neighbors(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    [
        x.checked_sub(1).map(|dx| (dx, y)),
        Some((x + 1, y)),
        y.checked_sub(1).map(|dy| (x, dy)),
        Some((x, y + 1)),
    ]
    .into_iter()
    .flatten()
}

fn main() -> Result<(), anyhow::Error> {
    let heightmap =
        io::stdin()
            .lock()
            .lines()
            .try_fold(Vec::<Vec<u8>>::new(), |mut buffer, line| {
                let values = {
                    let line = line?;

                    line.chars()
                        .try_fold(Vec::with_capacity(line.len()), |mut buffer, char| {
                            buffer.push(u8::try_from(
                                char.to_digit(10).ok_or(anyhow::anyhow!("unknown digit"))?,
                            )?);

                            Ok(buffer)
                        })?
                };

                buffer.push(values);

                Ok(buffer)
            })?;

    let mut lowpoints = HashSet::<(usize, usize)>::new();

    for (y, line) in heightmap.iter().enumerate() {
        for (x, &value) in line.iter().enumerate() {
            if value == 9 {
                continue;
            }

            if neighbors(x, y)
                .filter_map(|(nx, ny)| heightmap.get(ny).and_then(|line| line.get(nx)).copied())
                .all(|neighbor| neighbor > value)
            {
                lowpoints.insert((x, y));
            }
        }
    }

    println!(
        "Sum of the risk levels of all lowpoints: {}",
        lowpoints
            .iter()
            .map(|&(x, y)| 1 + u32::from(heightmap[y][x]))
            .sum::<u32>()
    );

    println!("Product of the size of the three largest bassins: {}", {
        let mut buffer = lowpoints
            .iter()
            .map(|&(x, y)| {
                fn visit(
                    x: usize,
                    y: usize,
                    heightmap: &[Vec<u8>],
                    bassin: &mut HashSet<(usize, usize)>,
                ) {
                    match heightmap.get(y).and_then(|line| line.get(x)) {
                        Some(&n) if n < 9 => {
                            bassin.insert((x, y));

                            for (nx, ny) in neighbors(x, y) {
                                if !bassin.contains(&(nx, ny)) {
                                    visit(nx, ny, heightmap, bassin);
                                }
                            }
                        }
                        _ => {}
                    }
                }

                let mut visited = HashSet::new();

                visit(x, y, &heightmap, &mut visited);

                visited.len()
            })
            .collect::<Vec<_>>();

        buffer.sort_unstable();

        buffer.iter().rev().take(3).product::<usize>()
    });

    Ok(())
}
