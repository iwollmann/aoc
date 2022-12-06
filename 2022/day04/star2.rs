use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = env::args().skip(1).take(1).next().unwrap();
    println!("Printing contents of: {:?}", filename);

    let mut count: u32 = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let pairs: Vec<_> = line.split(',').collect();

            let (a, b) = (pairs[0], pairs[1]);
            let pair1: Vec<u8> = a.split('-').map(|f| f.parse().unwrap()).collect();
            let pair2: Vec<u8> = b.split('-').map(|f| f.parse().unwrap()).collect();
            let (a1, a2) = (pair1[0], pair1[1]);
            let (b1, b2) = (pair2[0], pair2[1]);

            if (a2 <= b2 && a2 >= b1) || (b2 <= a2 && b2 >= a1) {
                count += 1;
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
