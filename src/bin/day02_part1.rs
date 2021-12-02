/// Day 2
/// Compute the position of the submarine after a series of movement, then multiply horizontal
/// position with depth.
use std::io::{self, BufRead};

fn main() -> Result<(), anyhow::Error> {
    let mut horizontal_pos = 0;
    let mut depth = 0;

    for line in io::stdin().lock().lines() {
        let line = line?;
        let (command, value) = line.split_once(" ").unwrap();
        let value = value.parse::<u64>()?;

        match command {
            "forward" => {
                horizontal_pos += value;
            }
            "down" => {
                depth += value;
            }
            "up" => {
                depth -= value;
            }
            _ => unreachable!("unknown command"),
        }
    }

    println!("Final horizontal position: {}", horizontal_pos);
    println!("Final depth: {}", depth);
    println!("depth * horizontal_pos = {}", depth * horizontal_pos);

    Ok(())
}
