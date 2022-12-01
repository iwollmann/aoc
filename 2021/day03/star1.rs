use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = env::args().skip(1).take(1).next().unwrap();
    println!("Printing contents of: {:?}", filename);

    if let Ok(lines) = read_lines(filename) {
        let mut numbers: [i32; 12] = [0; 12];
        for line in lines.flatten() {
            let bits: Vec<_> = line.chars().collect();

            for (idx, bit) in bits.iter().map(|x| x.to_digit(10).unwrap()).enumerate() {
                numbers[idx] += if bit > 0 { 1 } else { -1 };
            }
        }

        let mut epsilon: [u32; 12] = [0; 12];
        let mut gamma: [u32; 12] = [0; 12];
        for (idx, num) in numbers.iter().enumerate() {
            if num > &0 {
                gamma[idx] = 1;
                epsilon[idx] = 0;
            } else {
                gamma[idx] = 0;
                epsilon[idx] = 1;
            }
        }

        let epsilon_rate = epsilon
            .iter()
            .fold(0, |n, bit| n << 1 | if bit == &1 { 1 } else { 0 });
        let gamma_rate = gamma
            .iter()
            .fold(0, |n, bit| n << 1 | if bit == &1 { 1 } else { 0 });

        println!("Power Consumption: {:?}", epsilon_rate * gamma_rate);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
