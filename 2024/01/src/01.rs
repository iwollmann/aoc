use std::i32;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut arr1  = Vec::new();
    let mut arr2 = Vec::new();
    let mut total = 0;

    for line in input.lines() {
        let mut splitted = line.split_whitespace();
        let (line1, line2) = (splitted.next(), splitted.next());
        arr1.push(line1.unwrap().parse::<i32>().unwrap());
        arr2.push(line2.unwrap().parse::<i32>().unwrap());
    }

    arr1.sort();
    arr2.sort();

    for (index, item) in arr1.into_iter().enumerate() {
        let diff = item - arr2[index];
        if diff < 0 {
            total += diff*-1;
        } else {
            total += diff;
        }
    }

    return  total
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut arr2 = [0; 99999];

    let mut arr1  = Vec::new();
    let mut total = 0;

    for line in input.lines() {
        let mut splitted = line.split_whitespace().map( |x | x.parse::<usize>().ok());
        let first_val = splitted.next().flatten().unwrap();
        let second_val = splitted.next().flatten().unwrap();

        arr1.push(first_val);
        arr2[second_val] += 1;
    }

    for item in arr1 {
        total += arr2[item] * item;
    }

    return  total
}

