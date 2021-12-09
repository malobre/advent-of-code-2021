/// Day 8
/// I couldn't figure it out, I implemented a clever algorithm by `/u/4HbQ`.
/// <https://www.reddit.com/r/adventofcode/comments/rbj87a/comment/hnoyy04/>
use std::{
    collections::{BTreeMap, BTreeSet},
    io::{self, BufRead},
};

use anyhow::Ok;

fn main() -> Result<(), anyhow::Error> {
    let outputs_sum = io::stdin().lock().lines().try_fold(0_u32, |count, line| {
        let line = line?;

        let (signal, output) = line
            .rsplit_once('|')
            .ok_or(anyhow::anyhow!("expected a `|`"))?;

        let words_by_len = signal.split_whitespace().fold(
            BTreeMap::<usize, BTreeSet<char>>::new(),
            |mut buffer, word| {
                buffer.entry(word.len()).or_default().extend(word.chars());
                buffer
            },
        );

        let mut decoded_output: u32 = 0;

        for word in output.split_whitespace() {
            let word = word.chars().collect::<BTreeSet<char>>();

            let digit = match (
                word.len(),
                words_by_len
                    .get(&4)
                    .map(|four| four.intersection(&word).count()),
                words_by_len
                    .get(&2)
                    .map(|one| one.intersection(&word).count()),
            ) {
                (2, _, _) => 1,
                (3, _, _) => 7,
                (4, _, _) => 4,
                (7, _, _) => 8,
                (5, Some(2), _) => 2,
                (5, Some(3), Some(1)) => 5,
                (5, Some(3), Some(2)) => 3,
                (6, Some(4), _) => 9,
                (6, Some(3), Some(1)) => 6,
                (6, Some(3), Some(2)) => 0,
                params => anyhow::bail!("unable to decode word: {:?}", params),
            };

            decoded_output *= 10;
            decoded_output += digit;
        }

        Ok(count + decoded_output)
    })?;

    println!("Outputs sum: {}", outputs_sum);

    Ok(())
}
