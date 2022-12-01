use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = env::args().skip(1).take(1).next().unwrap();
    println!("Printing contents of: {:?}", filename);

    let mut snacks: Vec<i32> = Vec::new();
    let mut current = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let value = line.unwrap();
            println!("Printing value: {:?}", value);

            if let Ok(curr) = value.parse::<i32>() {
                current += curr;
            } else if value.is_empty() {
                snacks.push(current);
                current = 0;
            }
        }
    }

    snacks.sort_by(|a, b| b.cmp(a));
    let result: &i32 = &snacks[..=2].iter().sum();
    println!("Result: {:?}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
