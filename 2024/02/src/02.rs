aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut count = 0;
    for report in input.lines() {
        let levels = report.split_whitespace().map( |x | x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let mut level_result = true;
        let level_increasing = levels[0] < levels[1];
        for w in levels.windows(2) {
            level_result = match (w[0], w[1]) {
                (a,  b) if a == b =>  false,
                (a,  b) if a - b  > 3 || a - b < -3=> false,
                (a,  b) if (a < b)  != level_increasing => false,
                _ => true
            };
            if !level_result {
                break;
            }
        };

        if level_result {
            count +=1;
        }
    }

    return count
}

fn part_2(input: aoc::Input) -> impl ToString {
    0
}