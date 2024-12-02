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

fn is_ordered(slice: &[u32]) -> bool {
    let ascending = slice.windows(2).all(|w| w[0] <= w[1]);
    let descending = slice.windows(2).all(|w| w[0] >= w[1]);
    ascending || descending
}

fn diff_be_valid(slice: &[u32]) -> bool {
    slice.windows(2).all(|w| {
        let diff = if w[0] <= w[1] {
            w[1] - w[0]
        } else {
            w[0] - w[1]
        };
        diff >= 1 && diff <= 3
    })
}

fn is_safe_with_dampener(slice: &[u32]) -> bool {
    // already valid
    if is_ordered(slice) && diff_be_valid(slice) {
        return true;
    }

    for i in 0..slice.len() {
        let mut dampened = slice.to_vec();
        dampened.remove(i);

        if is_ordered(&dampened) && diff_be_valid(&dampened) {
            return true;
        }
    }

    false
}
