/// Day 10
use std::io::{self, BufRead};

use anyhow::Ok;

fn main() -> Result<(), anyhow::Error> {
    let (syntax_error_score, completion_scores) = io::stdin().lock().lines().try_fold(
        (0, Vec::new()),
        |(mut syntax_error_score, mut completion_scores), line| {
            let line = line?;

            let mut buffer = String::with_capacity(line.len());

            for char in line.chars() {
                let previous = buffer.chars().last();

                match (previous, char) {
                    (_, '(' | '[' | '{' | '<') => buffer.push(char),
                    (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') | (Some('<'), '>') => {
                        buffer.pop();
                    }
                    (_, ')' | ']' | '}' | '>') => {
                        syntax_error_score += match char {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => unreachable!(),
                        };

                        return Ok((syntax_error_score, completion_scores));
                    }
                    _ => anyhow::bail!("unknown chunk boundary `{}`", char),
                }
            }

            let mut completion_score = 0_u64;

            for char in buffer.chars().rev() {
                completion_score *= 5;

                completion_score += match char {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                };
            }

            completion_scores.push(completion_score);
            completion_scores.sort_unstable();

            Ok((syntax_error_score, completion_scores))
        },
    )?;

    println!("Syntax error score: {}", syntax_error_score);
    println!(
        "Median completion score: {}",
        completion_scores[completion_scores.len() / 2]
    );

    Ok(())
}
