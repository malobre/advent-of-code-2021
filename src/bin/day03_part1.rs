/// Day 3
/// Compute the gamma and epsilon rate, then multiply them together.
use std::io::{self, BufRead};

fn main() -> Result<(), anyhow::Error> {
    let (one_count, lines) = io::stdin().lock().lines().try_fold(
        ([0_u16; 12], 0_usize),
        |(mut one_count, lines), line| {
            for (index, bit) in line?.chars().enumerate() {
                match bit {
                    '1' => {
                        one_count[index] += 1;
                    }
                    '0' => {}
                    _ => anyhow::bail!("invalid bit `{}`", bit),
                }
            }

            anyhow::Ok((one_count, lines + 1))
        },
    )?;

    let gamma_rate = one_count
        .into_iter()
        .enumerate()
        .fold(0_u32, |acc, (index, count)| {
            if lines < usize::from(2 * count) {
                acc | (0b1 << (11 - index))
            } else {
                acc
            }
        });

    println!("  gamma rate: {:#014b}, {}", gamma_rate, gamma_rate);

    let epsilon_rate = !gamma_rate & 0b1111_1111_1111;

    println!("epsilon rate: {:#014b}, {}", epsilon_rate, epsilon_rate);

    let power_consumption = gamma_rate * epsilon_rate;

    println!("power consumption: {}", power_consumption);

    Ok(())
}
