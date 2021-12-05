/// Day 5
/// Compute thermal vent lines overlap.
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Vector2D {
    x: u16,
    y: u16,
}

impl Vector2D {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

fn main() -> Result<(), anyhow::Error> {
    let mut map = HashMap::<Vector2D, u8>::new();

    for line in io::stdin().lock().lines() {
        let (from, to) = line?
            .split_once(" -> ")
            .ok_or(anyhow::anyhow!("expected ` -> `"))
            .and_then(|(from, to)| {
                let from = from
                    .split_once(',')
                    .ok_or(anyhow::anyhow!("expected `,`"))?;
                let to = to.split_once(',').ok_or(anyhow::anyhow!("expected `,`"))?;

                let from = Vector2D::new(from.0.parse()?, from.1.parse()?);
                let to = Vector2D::new(to.0.parse()?, to.1.parse()?);

                Ok((from, to))
            })?;

        if from.x == to.x {
            let column = (from.y.min(to.y))..=(from.y.max(to.y));

            for y in column {
                *map.entry(Vector2D::new(from.x, y)).or_insert(0) += 1;
            }
        } else if from.y == to.y {
            let row = (from.x.min(to.x))..=(from.x.max(to.x));

            for x in row {
                *map.entry(Vector2D::new(x, from.y)).or_insert(0) += 1;
            }
        } else {
            let mut x = (from.x.min(to.x))..=(from.x.max(to.x));
            let mut y = (from.y.min(to.y))..=(from.y.max(to.y));

            loop {
                let x = if from.x > to.x {
                    x.next_back()
                } else {
                    x.next()
                };

                let y = if from.y > to.y {
                    y.next_back()
                } else {
                    y.next()
                };

                if let (Some(x), Some(y)) = (x, y) {
                    *map.entry(Vector2D::new(x, y)).or_insert(0) += 1;
                } else {
                    break;
                }
            }
        }
    }

    println!(
        "Number of points with an overlap >= 2: {}",
        map.values().filter(|overlap| **overlap >= 2).count()
    );

    Ok(())
}
