use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub enum Hand {
    A,
    B,
    C,
    X,
    Y,
    Z,
}

impl std::str::FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Hand::A),
            "B" => Ok(Hand::B),
            "C" => Ok(Hand::C),
            "X" => Ok(Hand::X),
            "Y" => Ok(Hand::Y),
            "Z" => Ok(Hand::Z),
            _ => Err(format!("'{}' is not a valid value for Hand", s)),
        }
    }
}

fn main() {
    let filename = env::args().skip(1).take(1).next().unwrap();
    println!("Printing contents of: {:?}", filename);

    let mut points = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let mut value = line.split_whitespace();

            let adv = value.next().unwrap().parse::<Hand>().unwrap();
            let my = value.next().unwrap().parse::<Hand>().unwrap();

            match (adv, my) {
                (Hand::A, Hand::X) => points += 0 + 3,
                (Hand::A, Hand::Y) => points += 3 + 1,
                (Hand::A, Hand::Z) => points += 6 + 2,
                (Hand::B, Hand::X) => points += 0 + 1,
                (Hand::B, Hand::Y) => points += 3 + 2,
                (Hand::B, Hand::Z) => points += 6 + 3,
                (Hand::C, Hand::X) => points += 0 + 2,
                (Hand::C, Hand::Y) => points += 3 + 3,
                (Hand::C, Hand::Z) => points += 6 + 1,
                _ => {}
            }
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
