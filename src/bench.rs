use super::Puzzle;

use serde::{Deserialize, Deserializer};
use std::time::Instant;

fn de<'de, D>(d: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(String::deserialize(d)?.as_bytes().to_vec())
}

#[allow(unused)]
#[derive(Deserialize)]
struct TestPuzzle {
    #[serde(deserialize_with = "de")]
    problem: Vec<u8>,
    solution: String,
}

#[allow(unused)]
pub fn puzzle() {
    let mut durations = Vec::with_capacity(500);

    for mut problem in serde_json::from_str::<Vec<TestPuzzle>>(
        &std::fs::read_to_string("test/puzzles-combined.json").unwrap(),
    )
    .unwrap()
    .iter()
    .map(|puzzle| Puzzle::new_from_string(&puzzle.problem))
    {
        let start = Instant::now();
        problem.solve();
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
