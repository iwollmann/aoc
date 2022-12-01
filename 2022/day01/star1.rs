use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = env::args().skip(1).take(1).next().unwrap();
    println!("Printing contents of: {:?}", filename);

    let mut max = 0;
    let mut current = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let value = line.unwrap();

            if let Ok(curr) = value.parse::<i32>() {
                current += curr;
            } else if value.is_empty() {
                if current > max {
                    max = current
                }
                current = 0;
            }
        }
    }

    println!("Result: {:?}", max);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
