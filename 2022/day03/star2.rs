use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = env::args().skip(1).take(1).next().unwrap();
    println!("Printing contents of: {:?}", filename);

    let mut points: u32 = 0;
    if let Ok(mut lines) = read_lines(filename) {
        while let (Some(h1), Some(h2), Some(h3)) = (lines.next(), lines.next(), lines.next()) {
            let first: HashSet<u8> = HashSet::from_iter(h1.unwrap().as_bytes().iter().cloned());
            let second: HashSet<u8> = HashSet::from_iter(h2.unwrap().as_bytes().iter().cloned());
            let third: HashSet<u8> = HashSet::from_iter(h3.unwrap().as_bytes().iter().cloned());

            // println!("Passing here: {:?} ", first,);
            second.iter().for_each(|item| {
                // println!("passing inside map: {:?} {:?}", item, points);
                if first.contains(item) && third.contains(item) {
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
