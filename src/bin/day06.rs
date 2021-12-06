/// Day 6
/// Compute the number of lanternfish after 80 and 256 days of growth.
use std::io::{self, BufRead};

fn main() -> Result<(), anyhow::Error> {
    // Each fish is modeled by the number of days until it creates offspring.
    let mut fishes = [0_u64; 9];

    for line in io::stdin().lock().lines() {
        for days_until_offspring in line?
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<usize>, _>>()?
        {
            fishes[days_until_offspring] += 1;
        }
    }

    for day in 0..256 {
        if day == 80 {
            println!(
                "Number of lanternfish after 80 days: {}",
                fishes.iter().sum::<u64>()
            );
        }

        fishes.rotate_left(1);

        // Reset the parent cycle.
        fishes[6] += fishes[8];
    }

    println!(
        "Number of lanternfish after 256 days: {}",
        fishes.iter().sum::<u64>()
    );

    Ok(())
}
