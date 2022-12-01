use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = env::args().skip(1).take(1).next().unwrap();
    println!("Printing contents of: {:?}", filename);

    let mut count = 0;
    if let Ok(lines) = read_lines(filename) {
        let mut p: i32 = i32::MAX;
        for line in lines {
            if let Ok(current) = line.unwrap().parse::<i32>() {
                if current > p {
                    count += 1;
                }
                p = current;
            }
        }
    }

    println!("Result: {:?}", count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
