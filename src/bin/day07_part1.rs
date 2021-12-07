/// Day 7
/// Crab in submarines?!
/// Compute the optimal position so that the least amount of fuel is used.
use std::io::{self, BufRead};

use anyhow::Ok;

fn main() -> Result<(), anyhow::Error> {
    let crab_subs = io::stdin()
        .lock()
        .lines()
        .try_fold(Vec::new(), |mut acc, line| {
            let mut values = line?
                .split(',')
                .map(str::parse)
                .collect::<Result<Vec<u32>, _>>()?;

            acc.append(&mut values);
            acc.sort_unstable();

            Ok(acc)
        })?;

    if crab_subs.is_empty() {
        anyhow::bail!("expected a non empty input");
    }

    let min = *crab_subs.first().unwrap();
    let max = *crab_subs.last().unwrap();

    // This is a very naive approach
    println!("optimal position fuel usage for part 1: {:?}", {
        let mut previous_fuel_usage = None;

        (min..=max)
            .map(|pos| p1_fuel_usage(&crab_subs, pos))
            .find(|&fuel_usage| {
                let increasing = matches!(
                    previous_fuel_usage,
                    Some(previous_fuel_usage) if fuel_usage > previous_fuel_usage
                );

                if !increasing {
                    previous_fuel_usage = Some(fuel_usage);
                }

                increasing
            });

        previous_fuel_usage
    });

    println!("optimal position fuel usage for part 2: {:?}", {
        let mut previous_fuel_usage = None;

        (min..=max)
            .map(|pos| p2_fuel_usage(&crab_subs, pos))
            .find(|&fuel_usage| {
                let increasing = matches!(
                    previous_fuel_usage,
                    Some(previous_fuel_usage) if fuel_usage > previous_fuel_usage
                );

                if !increasing {
                    previous_fuel_usage = Some(fuel_usage);
                }

                increasing
            });

        previous_fuel_usage
    });

    Ok(())
}

fn p1_fuel_usage(origin: &[u32], destination: u32) -> u32 {
    origin.iter().copied().fold(0, |acc, pos| {
        acc + u32::max(pos, destination) - u32::min(pos, destination)
    })
}

fn p2_fuel_usage(origin: &[u32], destination: u32) -> u32 {
    origin.iter().copied().fold(0, |acc, pos| {
        acc + (1..=(u32::max(pos, destination) - u32::min(pos, destination))).sum::<u32>()
    })
}
