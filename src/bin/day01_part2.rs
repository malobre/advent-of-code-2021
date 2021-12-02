/// Day 1
/// Count the number of times a depth measurement increases from the previous measurement, over a
/// sliding window of three measurements.
use std::{
    collections::VecDeque,
    io::{self, BufRead},
};

fn main() -> Result<(), anyhow::Error> {
    let mut increases = 0;
    let mut window = VecDeque::with_capacity(3);
    let mut previous_window_sum = None;

    for line in io::stdin().lock().lines() {
        let depth = line?.parse::<u64>()?;

        window.push_back(depth);

        if window.len() >= 3 {
            let sum: u64 = window.iter().sum();

            if let Some(previous_window_sum) = previous_window_sum {
                if sum > previous_window_sum {
                    increases += 1;
                }
            }

            previous_window_sum = Some(sum);
            window.pop_front();
        }
    }

    println!("Number of increases: {}", increases);

    Ok(())
}
