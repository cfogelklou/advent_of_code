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

--- Part Two ---
The engineers are surprised by the low number of safe reports until they realize they forgot to tell you about the Problem Dampener.

The Problem Dampener is a reactor-mounted module that lets the reactor safety systems tolerate a single bad level in what would otherwise be a safe report. It's like the bad level never happened!

Now, the same rules apply as before, except if removing a single level from an unsafe report would make it safe, the report instead counts as safe.

More of the above example's reports are now safe:

7 6 4 2 1: Safe without removing any level.
1 2 7 8 9: Unsafe regardless of which level is removed.
9 7 6 2 1: Unsafe regardless of which level is removed.
1 3 2 4 5: Safe by removing the second level, 3.
8 6 4 4 1: Safe by removing the third level, 4.
1 3 6 7 9: Safe without removing any level.
Thanks to the Problem Dampener, 4 reports are actually safe!

Update your analysis by handling situations where the Problem Dampener can remove a single level from unsafe reports. How many reports are now safe?

*/
use std::io::{self};
//use std::array;
mod utils;

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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn parse_data(data: String) -> Vec<Vec<i32>> {
    let data = utils::test_input_to_vec(data, false);
    let mut vecs: Vec<Vec<i32>> = Vec::new();
    for i in 0..data.len() {
        let words: Vec<&str> = data[i].split_whitespace().collect();
        let mut v = Vec::new();
        for j in 0..words.len() {
            v.push(utils::robust_to_int(words[j]));
        }
        vecs.push(v);
    }
    vecs
}

#[allow(dead_code)]
fn is_safe_report(vec: &Vec<i32>) -> bool {
    let is_inc_or_dec = all_same_direction(vec);
    let is_level_ok = levels_ok(vec);
    return is_inc_or_dec && is_level_ok;
}

#[allow(dead_code)]
fn count_safe_reports(vecs: Vec<Vec<i32>>) -> i32 {
    let mut safe_count: i32 = 0;
    for vec in vecs {
        if is_safe_report(&vec) {
            safe_count += 1;
        }
    }
    return safe_count;
}

#[allow(dead_code)]
fn count_safe_reports_remove(vecs: Vec<Vec<i32>>) -> i32 {
    let mut safe_count: i32 = 0;
    for vec in vecs {
        let mut safe = false;

        // If it's safe as is, then no need to try removing any elements
        if is_safe_report(&vec) {
            safe = true;
        } else {
            // Try removing each element and see if it's safe
            for i in 0..vec.len() {
                let mut vec2 = vec.clone();
                vec2.remove(i);

                // Is it safe now?
                if is_safe_report(&vec2) {
                    safe = true;
                    break;
                }
            }
        }
        if safe {
            safe_count += 1;
        }
    }
    return safe_count;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_safe_noses() {
        let data_bytes = String::from(
            "   7 6 4 2 1
                1 2 7 8 9
                9 7 6 2 1
                1 3 2 4 5
                8 6 4 4 1
                1 3 6 7 9",
        );

        let vecs = parse_data(data_bytes);
        let safe_count = count_safe_reports(vecs);

        // Expect safe_count to be 2
        assert_eq!(safe_count, 2);
        println!("Safe reports: {}", safe_count);
    }

    #[test]
    fn test_safe_noses_2() {
        let data_bytes = String::from(
            "   7 6 4 2 1
                1 2 7 8 9
                9 7 6 2 1
                1 3 2 4 5
                8 6 4 4 1
                1 3 6 7 9",
        );

        let vecs = parse_data(data_bytes);

        let safe_count2 = count_safe_reports_remove(vecs);
        // Expect safe_count to be 4
        assert_eq!(safe_count2, 4);

        println!("Safe reports: {}", safe_count2);
    }
}

pub fn main() -> io::Result<()> {
    let filename = if std::env::args().len() >= 2 {
        std::env::args().nth(1).unwrap()
    } else {
        String::from("day2/input.txt")
    };

    let data_bytes = std::fs::read_to_string(filename).unwrap();
    {
        let vecs = parse_data(data_bytes.clone());
        let safe_count = count_safe_reports(vecs);
        println!("Safe reports: {}", safe_count);
    }

    {
        let vecs = parse_data(data_bytes);
        let safe_count2 = count_safe_reports_remove(vecs);
        println!("Safe reports: {}", safe_count2);
    }

    Ok(())
}
