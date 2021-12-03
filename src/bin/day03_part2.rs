/// Day 3
/// Compute the life support rating from the oxygen generator & CO2 scrubber rating.
use std::io::{self, BufRead};

fn main() -> Result<(), anyhow::Error> {
    let input = io::stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|line| u16::from_str_radix(&line, 2))
        .collect::<Result<Vec<_>, _>>()?;

    let oxygen_generator_rating = {
        let mut buffer = input.clone();
        for index in (0..12).rev() {
            let one_count = buffer
                .iter()
                .fold(0_u16, |acc, line| match (line >> index) & 1 {
                    1 => acc + 1,
                    0 => acc,
                    _ => unreachable!("`x & 1` is either `0` or `1`"),
                });

            let one_is_more_or_equally_common = usize::from(2 * one_count) >= buffer.len();

            buffer = buffer
                .into_iter()
                .filter(|line| {
                    if one_is_more_or_equally_common {
                        (line >> index) & 1 == 1
                    } else {
                        (line >> index) & 1 == 0
                    }
                })
                .collect();

            if buffer.len() == 1 {
                break;
            }
        }

        buffer[0]
    };

    println!("oxygen generator rating: {}", oxygen_generator_rating);

    let co2_scrubber_rating = {
        let mut buffer = input;
        for index in (0..12).rev() {
            let one_count = buffer
                .iter()
                .fold(0_u16, |acc, line| match (line >> index) & 1 {
                    1 => acc + 1,
                    0 => acc,
                    _ => unreachable!("`x & 1` is either `0` or `1`"),
                });

            let zero_is_least_or_equally_common = usize::from(2 * one_count) >= buffer.len();

            buffer = buffer
                .into_iter()
                .filter(|line| {
                    if zero_is_least_or_equally_common {
                        (line >> index) & 1 == 0
                    } else {
                        (line >> index) & 1 == 1
                    }
                })
                .collect();

            if buffer.len() == 1 {
                break;
            }
        }

        buffer[0]
    };

    println!("co2 scrubber rating: {}", co2_scrubber_rating);

    println!(
        "life support rating: {}",
        u32::from(oxygen_generator_rating) * u32::from(co2_scrubber_rating)
    );

    Ok(())
}
