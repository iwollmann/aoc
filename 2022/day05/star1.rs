use core::num;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = env::args().skip(1).take(1).next().unwrap();
    println!("Printing contents of: {:?}", filename);

    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); 10];
    if let Ok(lines) = read_lines(filename) {
        lines.for_each(|line| match line.unwrap() {
            a if a.is_empty() => (),
            a if a[..2] == *" 1" => (),
            a if a[..4] == *"move" => {
                let nums: Vec<usize> = a.split(' ').flat_map(|f| f.parse::<usize>()).collect();
                println!("Movement: {:?} {:?}", stacks, nums);

                for _ in 0..nums[0] {
                    if let Some(val) = stacks[nums[1] - 1].pop_front() {
                        stacks[nums[2] - 1].push_front(val)
                    }
                }
            }
            a => {
                let a: Vec<char> = a.chars().collect();
                (0..stacks.len()).for_each(|i| {
                    if let Some(inner) = a.get(1 + i * 4) {
                        if *inner != ' ' {
                            stacks[i].push_back(*inner)
                        }
                    }
                });
            }
        });
    }

    println!(
        "Result: {:?}",
        stacks.iter().flat_map(|s| s.front()).collect::<String>()
    );
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
