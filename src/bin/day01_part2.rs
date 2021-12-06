/// Day 1
/// Count the number of times a depth measurement increases from the previous measurement, over a
/// sliding window of three measurements.
use std::io::{self, BufRead};

use anyhow::Ok;

fn main() -> Result<(), anyhow::Error> {
    let (_last_window_sum, increases) = io::stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .windows(3)
        .try_fold((None, 0_u64), |(previous_window_sum, increases), window| {
            let window_sum = window
                .iter()
                .try_fold(0_u64, |sum, depth| Ok(sum + depth.parse::<u64>()?))?;

            match previous_window_sum {
                Some(previous_window_sum) if window_sum > previous_window_sum => {
                    Ok((Some(window_sum), increases + 1))
                }
                _ => Ok((Some(window_sum), increases)),
            }
        })?;

    println!("Number of increases: {}", increases);

    Ok(())
}
