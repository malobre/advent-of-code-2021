/// Day 1
/// Count the number of times a depth measurement increases from the previous measurement.
use std::io::{self, BufRead};

fn main() -> Result<(), anyhow::Error> {
    let mut increases = 0;
    let mut previous_depth = None;

    for line in io::stdin().lock().lines() {
        let depth = line?.parse::<u64>()?;

        if let Some(previous_depth) = previous_depth {
            if depth > previous_depth {
                increases += 1;
            }
        }

        previous_depth = Some(depth);
    }

    println!("Number of increases: {}", increases);

    Ok(())
}
