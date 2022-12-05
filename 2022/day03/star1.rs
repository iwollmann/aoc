use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = env::args().skip(1).take(1).next().unwrap();
    println!("Printing contents of: {:?}", filename);

    let mut points: u32 = 0;
    if let Ok(lines) = read_lines(filename) {
        for mut line in lines.flatten() {
            let size = line.len();
            let (h1, h2) = line.split_at_mut(size / 2);
            let first: HashSet<u8> = HashSet::from_iter(h1.as_bytes().iter().cloned());
            let second: HashSet<u8> = HashSet::from_iter(h2.as_bytes().iter().cloned());

            second.iter().for_each(|item| {
                if first.contains(item) {
                    match item {
                        x if x <= &b'Z' => points += (x - b'A' + 1 + 26) as u32,
                        x if x >= &b'a' => points += (x - b'a' + 1) as u32,
                        _ => {}
                    }
                }
            });
        }
    }

    println!("Result: {:?}", points);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
