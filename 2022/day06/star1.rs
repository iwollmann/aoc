use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = env::args().skip(1).take(1).next().unwrap();
    println!("Printing contents of: {:?}", filename);

    let mut letters = Vec::new();
    if let Ok(mut lines) = read_lines(filename) {
        if let Some(line) = lines.next() {
            for (idx, letter) in line.unwrap().chars().enumerate() {
                if let Some(index) = letters.iter().position(|&s| s == letter) {
                    let (left, right) = letters.split_at(index + 1);

                    letters = right.to_vec();
                }

                letters.push(letter);

                if letters.len() == 4 {
                    println!("Result: {:?}", idx + 1);
                    break;
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
