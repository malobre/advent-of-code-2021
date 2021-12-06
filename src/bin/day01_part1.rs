/// Day 1
/// Count the number of times a depth measurement increases from the previous measurement.
use std::io::{self, BufRead};

use anyhow::Ok;

fn main() -> Result<(), anyhow::Error> {
    let (_last_depth, increases) = io::stdin().lock().lines().try_fold(
        (None, 0_u32),
        |(previous_depth, increases), depth| {
            let depth = depth?.parse::<u64>()?;

            match previous_depth {
                Some(previous_depth) if depth > previous_depth => Ok((Some(depth), increases + 1)),
                _ => Ok((Some(depth), increases)),
            }
        },
    )?;

    println!("Number of increases: {}", increases);

    Ok(())
}
