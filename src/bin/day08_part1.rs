/// Day 8
use std::io::{self, BufRead};

use anyhow::Ok;

fn main() -> Result<(), anyhow::Error> {
    let unique_digits = io::stdin().lock().lines().try_fold(0, |count, line| {
        let line = line?;

        let (_, output_values) = line
            .rsplit_once('|')
            .ok_or(anyhow::anyhow!("expected a `|`"))?;

        let identified_values = output_values
            .trim()
            .split_whitespace()
            .fold(0, |acc, segments| match segments.len() {
                2 | 4 | 3 | 7 => acc + 1,
                _ => acc,
            });

        Ok(count + identified_values)
    })?;

    println!("Number of 1, 4, 7 & 8: {}", unique_digits);

    Ok(())
}
