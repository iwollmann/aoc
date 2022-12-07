use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = env::args().skip(1).take(1).next().unwrap();
    println!("Printing contents of: {:?}", filename);

    if let Ok(mut lines) = read_lines(filename) {
        if let Some(line) = lines.next() {
            println!("Result: {:?}", solve(line.unwrap()));
        }
    }
}

pub fn solve(input: String) -> u32 {
    let mut letters = Vec::new();
    for (idx, letter) in input.chars().enumerate() {
        if let Some(index) = letters.iter().position(|&s| s == letter) {
            let (_, right) = letters.split_at(index + 1);

            letters = right.to_vec();
        }

        letters.push(letter);

        if letters.len() == 14 {
            return (idx + 1) as u32;
        }
    }

    0
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn solve_test() {
        let samples = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        for (input, expect) in samples {
            assert_eq!(
                solve(input.to_string()),
                expect,
                "Testing {} should be {}",
                input,
                expect
            );
        }
    }
}
