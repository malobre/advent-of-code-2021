/// Day 2
/// Compute the position of the submarine after a series of movement, then multiply horizontal
/// position with depth.
use std::io::{self, BufRead};

use anyhow::Ok;

fn main() -> Result<(), anyhow::Error> {
    let (depth, horizontal_pos) =
        io::stdin()
            .lock()
            .lines()
            .try_fold((0, 0), |(depth, horizontal_pos), input| {
                let input = input?;
                let (command, value) = input
                    .split_once(" ")
                    .ok_or(anyhow::anyhow!("expected `<command> <arg>`"))?;
                let value = value.parse::<u64>()?;

                match command {
                    "forward" => Ok((depth, horizontal_pos + value)),
                    "down" => Ok((depth + value, horizontal_pos)),
                    "up" => Ok((depth - value, horizontal_pos)),
                    _ => anyhow::bail!("unknown command `{}`", command),
                }
            })?;

    println!("Final horizontal position: {}", horizontal_pos);
    println!("Final depth: {}", depth);
    println!("depth * horizontal_pos = {}", depth * horizontal_pos);

    Ok(())
}
