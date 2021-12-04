/// Day 4
/// Bingo !
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
struct BingoCell {
    value: u8,
    marked: bool,
}

#[derive(Debug, Clone, Copy)]
struct BingoBoard {
    cells: [BingoCell; 25],
}

impl BingoBoard {
    /// Mark a number and returns the board score if a row or a column is fully marked.
    pub fn mark(&mut self, number: u8) -> Option<u32> {
        let mut marked_index = None;

        for (index, mut cell) in self.cells.iter_mut().enumerate() {
            if cell.value == number {
                cell.marked = true;
                marked_index = Some(index);
                break;
            }
        }

        if let Some(marked_index) = marked_index {
            let row_start_index = marked_index - marked_index % 5;
            let row_end_index = row_start_index + 5;

            if let Some(row) = self.cells.get(row_start_index..row_end_index) {
                if row.iter().all(|cell| cell.marked) {
                    return Some(self.score(number));
                }
            }

            if self
                .cells
                .iter()
                .enumerate()
                .filter(|(index, _)| index % 5 == marked_index % 5)
                .all(|(_, cell)| cell.marked)
            {
                return Some(self.score(number));
            }
        }

        None
    }

    fn score(&self, last_marked_number: u8) -> u32 {
        let unmarked_sum = self
            .cells
            .iter()
            .filter_map(|cell| {
                if cell.marked {
                    None
                } else {
                    Some(u32::from(cell.value))
                }
            })
            .sum::<u32>();

        unmarked_sum * u32::from(last_marked_number)
    }
}

fn main() -> Result<(), anyhow::Error> {
    let drawing_order = {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        buffer
            .trim_end()
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<u8>, _>>()?
    };

    let mut boards = Vec::new();

    // Build the boards
    {
        let mut buffer = Vec::with_capacity(25);
        for line in io::stdin().lock().lines() {
            buffer.append(
                &mut line?
                    .split_whitespace()
                    .map(str::parse)
                    .collect::<Result<Vec<u8>, _>>()?,
            );

            if buffer.len() >= 25 {
                boards.push(BingoBoard {
                    cells: buffer
                        .drain(0..25)
                        .map(|value| BingoCell {
                            value,
                            marked: false,
                        })
                        .collect::<Vec<_>>()
                        .as_slice()
                        .try_into()?,
                });
            }
        }
    }

    let winning_boards_score = {
        let mut buffer = Vec::new();

        for number in drawing_order {
            // This could be replaced with unstable `Vec::drain_filter`.
            let mut i = 0;
            while i < boards.len() {
                if let Some(score) = boards[i].mark(number) {
                    buffer.push(score);
                    boards.remove(i);
                } else {
                    i += 1;
                }
            }
        }

        buffer
    };

    println!(
        "Score of the first board to win: {:?}",
        winning_boards_score.first()
    );
    println!(
        "Score of the last board to win: {:?}",
        winning_boards_score.last()
    );

    Ok(())
}
