/// Day 7
/// Crab in submarines?!
/// Compute the optimal position so that the least amount of fuel is used.
use std::io::{self, BufRead};

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

            Ok::<_, anyhow::Error>(acc)
        })?;

    println!(
        "optimal position fuel usage for part 1: {:?}",
        p1_fuel_usage(&crab_subs, crab_subs[crab_subs.len() / 2])
    );

    println!("optimal position fuel usage for part 2: {:?}", {
        let average = crab_subs.iter().sum::<u32>() / u32::try_from(crab_subs.len())?;

        match crab_subs.binary_search_by(|pos| pos.cmp(&average)) {
            Ok(index) => p2_fuel_usage(&crab_subs, crab_subs[index]),
            Err(index) => [index - 1, index]
                .into_iter()
                .map(|index| p2_fuel_usage(&crab_subs, crab_subs[index]))
                .min()
                .expect("array is not empty"),
        }
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
