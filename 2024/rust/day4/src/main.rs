/*
--- Day 4: Ceres Search ---
"Looks like the Chief's not here. Next!" One of The Historians pulls out a device and pushes the only button on it. After a brief flash, you recognize the interior of the Ceres monitoring station!

As the search for the Chief continues, a small Elf who lives on the station tugs on your shirt; she'd like to know if you could help her with her word search (your puzzle input). She only has to find one word: XMAS.

This word search allows words to be horizontal, vertical, diagonal, written backwards, or even overlapping other words. It's a little unusual, though, as you don't merely need to find one instance of XMAS - you need to find all of them. Here are a few ways XMAS might appear, where irrelevant characters have been replaced with .:


..X...
.SAMX.
.A..A.
XMAS.S
.X....
The actual word search will be full of letters instead. For example:

MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
In this word search, XMAS occurs a total of 18 times; here's the same word search again, but where letters not involved in any XMAS have been replaced with .:

....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX
Take a look at the little Elf's word search. How many times does XMAS appear?

--- Part Two ---
The Elf looks quizzically at you. Did you misunderstand the assignment?

Looking for the instructions, you flip over the word search to find that this isn't actually an XMAS puzzle; it's an X-MAS puzzle in which you're supposed to find two MAS in the shape of an X. One way to achieve that is like this:

M.S
.A.
M.S
Irrelevant characters have again been replaced with . in the above diagram. Within the X, each MAS can be written forwards or backwards.

Here's the same example from before, but this time all of the X-MASes have been kept instead:

.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
In this example, an X-MAS appears 9 times.

Flip the word search from the instructions back over to the word search side and try again. How many times does an X-MAS appear?

*/
#[allow(unused_imports)]
use std::ffi::CString;
use std::io::{self};
//use std::array;
mod utils;

// Function now takes a reference to the grid instead of consuming it
#[allow(dead_code)]
fn get_letter(grid: &Vec<Vec<char>>, i: i32, j: i32) -> char {
    let width = grid[0].len();
    if i < 0 || i >= grid.len() as i32 {
        return '.';
    } else if j < 0 || j >= width as i32 {
        return '.';
    }
    return grid[i as usize][j as usize];
}

#[allow(dead_code)]
fn search_for_letter_around_point(
    grid: &Vec<Vec<char>>,
    remaining_word: &Vec<char>,
    i: i32,
    j: i32,
    directions: &Vec<(i32, i32)>,
) -> usize {
    let letter = remaining_word[0];
    let remaining_word = remaining_word[1..].to_vec();
    let mut count = 0;
    for d in directions.iter() {
        let x = i + d.0;
        let y = j + d.1;
        if get_letter(grid, x, y) == letter {
            if remaining_word.len() == 0 {
                count += 1;
            } else {
                let this_direction = vec![(d.0, d.1)];
                count += search_for_letter_around_point(
                    grid,
                    &remaining_word.clone(),
                    x,
                    y,
                    &this_direction,
                );
            }
        }
    }

    return count;
}

// Update search_letter to pass a reference to get_letter
#[allow(dead_code)]
fn search_letter0(grid: Vec<Vec<char>>, word: &str) -> usize {
    let directions = vec![
        (0, 1),
        (1, 0),
        (1, 1),
        (1, -1),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (-1, 1),
    ];
    let remaining_word: Vec<char> = word.chars().collect();
    let mut count = 0;
    let width = grid[0].len();
    let letter = remaining_word[0];
    let remaining_word = remaining_word[1..].to_vec();
    for i in 0..grid.len() {
        for j in 0..width {
            if get_letter(&grid, i as i32, j as i32) == letter {
                count += search_for_letter_around_point(
                    &grid,
                    &remaining_word,
                    i as i32,
                    j as i32,
                    &directions,
                );
            }
        }
    }
    return count;
}

#[allow(dead_code)]
fn search_x_mas(grid: Vec<Vec<char>>) -> usize {
    let mut count = 0;
    let width = grid[0].len();
    for i in 0..grid.len() {
        for j in 0..width {
            if get_letter(&grid, i as i32, j as i32) == 'A' {
                let letter_up_left = get_letter(&grid, i as i32 - 1, j as i32 - 1);
                let letter_down_right = get_letter(&grid, i as i32 + 1, j as i32 + 1);
                if (letter_up_left == 'M' && letter_down_right == 'S')
                    || (letter_up_left == 'S' && letter_down_right == 'M')
                {
                    let letter_up_right = get_letter(&grid, i as i32 - 1, j as i32 + 1);
                    let letter_down_left = get_letter(&grid, i as i32 + 1, j as i32 - 1);
                    if (letter_up_right == 'M' && letter_down_left == 'S')
                        || (letter_up_right == 'S' && letter_down_left == 'M')
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    return count;
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_crossword_1() {
        let data_bytes = String::from(
            "....XXMAS.
            .SAMXMS...
            ...S..A...
            ..A.A.MS.X
            XMASAMX.MM
            X.....XA.A
            S.S.S.S.SS
            .A.A.A.A.A
            ..M.M.M.MM
            .X.X.XMASX",
        );
        println!("Data bytes: {}", data_bytes);
        let grid = utils::create_grid_from_string(data_bytes);
        let count2 = search_letter0(grid, "XMAS");
        assert_eq!(count2, 18);
    }

    #[test]
    fn test_crossword_2() {
        let data_bytes = String::from(
            "MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX",
        );
        println!("Data bytes: {}", data_bytes);
        let grid = utils::create_grid_from_string(data_bytes);
        let count2 = search_letter0(grid, "XMAS");
        assert_eq!(count2, 18);
    }

    #[test]
    fn test_crossword_x_mas() {
        let data_bytes = String::from(
            "MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX",
        );
        println!("Data bytes: {}", data_bytes);
        let grid = utils::create_grid_from_string(data_bytes);
        let count2 = search_x_mas(grid);
        assert_eq!(count2, 9);
    }
}

pub fn main() -> io::Result<()> {
    let filename = if std::env::args().len() >= 2 {
        std::env::args().nth(1).unwrap()
    } else {
        String::from("day4/input.txt")
    };
    let data_bytes = std::fs::read_to_string(filename).unwrap();
    println!("Data bytes: {}", data_bytes);
    let grid = utils::create_grid_from_string(data_bytes);
    let word_search = search_letter0(grid.clone(), "XMAS");

    println!("wordsearch: {}", word_search);

    let num_x_mas = search_x_mas(grid);
    println!("num_x_mas: {}", num_x_mas);

    Ok(())
}
