use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), std::io::Error> {
    let input = File::open("assets/input.txt")?;
    let input = BufReader::new(input);

    let mut seen = HashSet::new();

    let deltas: Vec<i64> = input
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();

    let mut sum = 0i64;
    for freq in deltas.iter().cycle() {
        sum += freq;
        if !seen.insert(sum) {
            println!("repeated: {}", sum);
            break;
        }
    }

    Ok(())
}
