use lazy_static::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Claim {
    idx: u32,
    top: u32,
    left: u32,
    height: u32,
    width: u32,
}

fn process_claim(claim_text: &str) -> Result<Claim, std::num::ParseIntError> {
    lazy_static! {
        static ref RX: Regex = Regex::new(
            r"#(?P<id>\d+) @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)x(?P<height>\d+)"
        )
        .unwrap();
    }

    let claim_parts = RX.captures(claim_text).unwrap();

    Ok(Claim {
        idx: claim_parts["id"].parse()?,
        left: claim_parts["left"].parse()?,
        top: claim_parts["top"].parse()?,
        width: claim_parts["width"].parse()?,
        height: claim_parts["height"].parse()?,
    })
}

fn claim_filter(claim: &Claim, fabric: &mut HashMap<(u32, u32), u32>) -> bool {
    for x in claim.left..(claim.left + claim.width) {
        for y in claim.top..(claim.top + claim.height) {
            if *fabric.get(&(x, y)).unwrap_or(&0u32) != 1 {
                return false;
            }
        }
    }

    true
}

fn main() -> Result<(), std::io::Error> {
    let input = File::open("assets/input.txt")?;
    let input = BufReader::new(input);

    let claims = input
        .lines()
        .filter_map(Result::ok)
        .map(|s| process_claim(s.as_str()))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut fabric: HashMap<(u32, u32), u32> = HashMap::new();

    for claim in claims.iter() {
        for x in claim.left..(claim.left + claim.width) {
            for y in claim.top..(claim.top + claim.height) {
                *fabric.entry((x, y)).or_insert(0u32) += 1;
            }
        }
    }

    // first part
    let overlaps = fabric.values().filter(|c| **c >= 2u32).count();

    println!("number of overlapping claims: {}", overlaps);

    // second part
    let non_overlapping: Vec<_> = claims
        .iter()
        .filter(|claim| claim_filter(&claim, &mut fabric))
        .map(|claim| claim.idx)
        .collect();

    println!("indexes of non-overlapping claims: {:?}", non_overlapping);

    Ok(())
}
