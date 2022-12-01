use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = env::args().skip(1).take(1).next().unwrap();
    println!("Printing contents of: {:?}", filename);

    let mut count = 0;
    if let Ok(lines) = read_lines(filename) {
        let mut numbers = VecDeque::new();
        for line in lines.flatten() {
            if let Ok(current) = line.parse::<i32>() {
                numbers.push_back(current);

                if numbers.len() == 4 {
                    match numbers.pop_front().unwrap().cmp(&current) {
                        std::cmp::Ordering::Less => count += 1,
                        _ => {}
                    }
                }
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

// - value
// - count
// - previous

// {
//     value: 1
//     count: 1
//     previous: null
// },

// 171 - A
// 173 - A B
// 174 - A B
// 163 -   B
// 161
