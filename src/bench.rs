use super::Puzzle;

use std::time::Instant;

pub fn puzzle() {
    let mut durations = Vec::with_capacity(500);

    for problem in std::fs::read_to_string("test/puzzles-combined.txt")
        .unwrap()
        .lines()
    {
        let mut puzzle = Puzzle::new_from_string(problem.as_bytes());
        let start = Instant::now();
        puzzle.solve();
        durations.push(start.elapsed().as_micros());
    }

    durations.sort_unstable();
    println!(
        "min => {} us/iter, max => {} us/iter",
        durations.first().unwrap(),
        durations.last().unwrap(),
    );
    println!("avg => {} us/iter", durations.iter().sum::<u128>() / 500);
}
