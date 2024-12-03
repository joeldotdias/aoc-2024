use std::fs;

fn main() {
    let safe: u32 = fs::read_to_string("input/day2")
        .unwrap()
        .lines()
        .fold(0, |acc, line| {
            let level: Vec<u32> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            // part 1
            // if !(is_ordered(&level)) {
            //     return acc;
            // }
            //
            // for nums in level.windows(2) {
            //     if let [prev, curr] = nums {
            //         if !(1..=3).contains(&curr.abs_diff(*prev)) {
            //             return acc;
            //         }
            //     }
            // }

            // part 2
            if !is_safe_with_dampener(&level) {
                return acc;
            }

            acc + 1
        });

    println!("Safe: {safe}");
}

fn is_ordered(level: &[u32]) -> bool {
    let ascending = level.windows(2).all(|w| w[0] <= w[1]);
    let descending = level.windows(2).all(|w| w[0] >= w[1]);
    ascending || descending
}

fn diff_be_valid(level: &[u32]) -> bool {
    level.windows(2).all(|w| {
        let diff = if w[0] <= w[1] {
            w[1] - w[0]
        } else {
            w[0] - w[1]
        };
        (1..=3).contains(&diff)
    })
}

fn is_safe_with_dampener(level: &[u32]) -> bool {
    // already valid
    if is_ordered(level) && diff_be_valid(level) {
        return true;
    }

    for i in 0..level.len() {
        let mut dampened = level.to_vec();
        dampened.remove(i);

        if is_ordered(&dampened) && diff_be_valid(&dampened) {
            return true;
        }
    }

    false
}
