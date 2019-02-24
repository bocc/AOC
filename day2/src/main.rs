use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// two ids are equal if they differ only in one character
fn id_eq(first: &String, second: &String) -> bool {
    let mut diff = 0u32;

    let second = second.as_bytes();

    for (i, b) in first.as_bytes().iter().enumerate() {
        if *b != second[i] {
            diff += 1;
        }
    }

    diff == 1
}

fn main() -> Result<(), std::io::Error> {
    let input = File::open("assets/input.txt")?;
    let input = BufReader::new(input);

    let lines: Vec<_> = input.lines().filter_map(Result::ok).collect();

    'outer: for line in lines.iter() {
        for line2 in lines.iter() {
            if id_eq(line, line2) {
                let line2 = line2.as_bytes();
                let mut v = Vec::with_capacity(line.len());

                for (i, c) in line.as_bytes().iter().enumerate() {
                    if *c == line2[i] {
                        v.push(*c);
                    }
                }

                println!("{}", String::from_utf8(v).unwrap());
                break 'outer;
            }
        }
    }

    Ok(())
}
