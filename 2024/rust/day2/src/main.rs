/*
--- Day 2: Red-Nosed Reports ---
Fortunately, the first location The Historians want to search isn't a long walk from the Chief Historian's office.

While the Red-Nosed Reindeer nuclear fusion/fission plant appears to contain no sign of the Chief Historian, the engineers there run up to you as soon as they see you. Apparently, they still talk about the time Rudolph was saved through molecular synthesis from a single electron.

They're quick to add that - since you're already here - they'd really appreciate your help analyzing some unusual data from the Red-Nosed reactor. You turn to check if The Historians are waiting for you, but they seem to have already divided into groups that are currently searching every corner of the facility. You offer to help with the unusual data.

The unusual data (your puzzle input) consists of many reports, one report per line. Each report is a list of numbers called levels that are separated by spaces. For example:

7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
This example data contains six reports each containing five levels.

The engineers are trying to figure out which reports are safe. The Red-Nosed reactor safety systems can only tolerate levels that are either gradually increasing or gradually decreasing. So, a report only counts as safe if both of the following are true:

The levels are either all increasing or all decreasing.
Any two adjacent levels differ by at least one and at most three.
In the example above, the reports can be found safe or unsafe by checking those rules:

7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
So, in this example, 2 reports are safe.

Analyze the unusual data from the engineers. How many reports are safe?


*/
use std::io::{self};
//use std::array;
mod utils;
use std::cmp::Ordering;

#[allow(dead_code)]
fn all_same_direction(vec: &Vec<i32>) -> bool {
    // Return true if all elements are either increasing or decreasing
    let mut increasing: bool = false;
    let mut decreasing: bool = false;
    let mut same: bool = false;
    for i in 0..vec.len() - 1 {
        let a = vec[i];
        let b = vec[i + 1];
        if a < b {
            increasing = true;
        } else if a > b {
            decreasing = true;
        } else {
            same = true;
        }
    }
    return (increasing ^ decreasing) && !same;
}

fn levels_ok(vec: &Vec<i32>) -> bool {
    // Return true if all elements are either increasing or decreasing
    let min_diff: i32 = 1;
    let max_diff: i32 = 3;
    let mut ok = true;
    for i in 0..vec.len() - 1 {
        let diff = (vec[i] - vec[i + 1]).abs();
        if diff < min_diff || diff > max_diff {
            ok = false;
        }
    }
    return ok;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn safe_noses() {
        let data_bytes = String::from(
            "   7 6 4 2 1
                1 2 7 8 9
                9 7 6 2 1
                1 3 2 4 5
                8 6 4 4 1
                1 3 6 7 9",
        );
        let data = utils::test_input_to_vec(data_bytes, false);

        // Create an array of 5 Vec<i32> to hold the data
        let mut vecs: Vec<Vec<i32>> = Vec::new();

        for i in 0..data.len() {
            let words: Vec<&str> = data[i].split_whitespace().collect();
            let mut v = Vec::new();
            for j in 0..words.len() {
                v.push(utils::robust_to_int(words[j]));
            }
            vecs.push(v);
        }

        // For each Vec<i32>, are they the same?
        let mut safe_count: i32 = 0;
        for i in 0..vecs.len() {
            let vec_a = &vecs[i];
            // print vec_a
            println!("{:?}", vec_a);
            let is_inc_or_dec = all_same_direction(vec_a);
            let is_level_ok = levels_ok(vec_a);
            if (is_inc_or_dec && is_level_ok) {
                safe_count += 1;
            }
        }

        // Expect distance to be 2
        assert_eq!(safe_count, 2);
    }
}

pub fn main() -> io::Result<()> {
    let filename = if std::env::args().len() >= 2 {
        std::env::args().nth(1).unwrap()
    } else {
        String::from("day1/input.txt")
    };
    let data_bytes = std::fs::read_to_string(filename).unwrap();
    let data = utils::test_input_to_vec(data_bytes, false);
    // Convert each string to an integer

    Ok(())
}
