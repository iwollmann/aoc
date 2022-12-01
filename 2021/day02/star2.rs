use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = env::args().skip(1).take(1).next().unwrap();
    println!("Printing contents of: {:?}", filename);

    let mut x = 0;
    let mut depth = 0;
    let mut aim = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let commands: Vec<_> = line.split(" ").collect();

            let value = commands[1].parse::<i64>();
            let command = commands[0];

            match (command, value) {
                ("forward", Ok(value)) => {
                    x += value;
                    depth += value * aim;
                }
                ("up", Ok(value)) => aim -= value,
                ("down", Ok(value)) => aim += value,
                _ => {}
            }
        }

        println!("Commands: {:?}", depth * x);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
