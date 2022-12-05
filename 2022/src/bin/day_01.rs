use advent_of_code_2022 as util;

fn main() {
    const PREFIX_COUNT: usize = 3;

    let mut max = Vec::with_capacity(PREFIX_COUNT);
    let mut curr = 0u64;

    util::each_line("input/day_01.txt", |line| {
        if line.len() == 0 {
            if max.len() == 0 {
                max.push(curr);
            } else if max[max.len() - 1] < curr {
                if max.len() == PREFIX_COUNT {
                    max.pop();
                }
                max.push(curr);
                max.sort_unstable_by(|a, b| b.cmp(a));
            }

            curr = 0;
        } else {
            curr += u64::from_str_radix(line, 10).unwrap();
        }
    });

    println!("Part 1: {}", max[0]);
    println!("Part 2: {}", max.iter().sum::<u64>());
}
