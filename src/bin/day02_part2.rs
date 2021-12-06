/// Day 2
/// Compute the position of the submarine after a series of movement, then multiply horizontal
/// position with depth.
use std::io::{self, BufRead};

use anyhow::Ok;

fn main() -> Result<(), anyhow::Error> {
    let (depth, horizontal_pos, _aim) =
        io::stdin()
            .lock()
            .lines()
            .try_fold((0, 0, 0), |(depth, horizontal_pos, aim), input| {
                let input = input?;
                let (command, value) = input
                    .split_once(" ")
                    .ok_or(anyhow::anyhow!("expected `<command> <arg>`"))?;
                let value = value.parse::<u64>()?;

                match command {
                    "forward" => Ok((depth + aim * value, horizontal_pos + value, aim)),
                    "down" => Ok((depth, horizontal_pos, aim + value)),
                    "up" => Ok((depth, horizontal_pos, aim - value)),
                    _ => anyhow::bail!("unknown command `{}`", command),
                }
            })?;

    println!("Final horizontal position: {}", horizontal_pos);
    println!("Final depth: {}", depth);
    println!("depth * horizontal_pos = {}", depth * horizontal_pos);

    Ok(())
}
