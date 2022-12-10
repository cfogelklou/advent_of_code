use std::io::{self};
mod utils;

#[allow(dead_code)]
fn get_head_movement(v: &Vec<String>) -> i32 {
    return 0
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn cpu_test() {
        let raw_string: String = "addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop"
        .to_string();
        let v: Vec<String> = utils::test_input_to_vec(raw_string, true);
        assert_ne!(0, v.len());

        // During the 20th cycle, register X has the value 21, so the signal strength is 20 * 21 = 420. 
        // (The 20th cycle occurs in the middle of the second addx -1, so the value of register X is the starting value, 1, plus all of the other addx values up to that point: 1 + 15 - 11 + 6 - 3 + 5 - 1 - 8 + 13 + 4 = 21.)
        // During the 60th cycle, register X has the value 19, so the signal strength is 60 * 19 = 1140.
        // During the 100th cycle, register X has the value 18, so the signal strength is 100 * 18 = 1800.
        // During the 140th cycle, register X has the value 21, so the signal strength is 140 * 21 = 2940.
        // During the 180th cycle, register X has the value 16, so the signal strength is 180 * 16 = 2880.
        // During the 220th cycle, register X has the value 18, so the signal strength is 220 * 18 = 3960.

    }

}

fn main() -> io::Result<()> {
    use std::io::BufRead;

    let filename = if std::env::args().len() >= 2 {
        std::env::args().nth(1).unwrap()
    } else {
        String::from("input.txt")
    };
    let file = std::io::BufReader::new(
        std::fs::File::open(<String as AsRef<std::path::Path>>::as_ref(&filename)).unwrap(),
    );
    let mut v: Vec<String> = Vec::new();
    for (_, line) in file.lines().enumerate() {
        let l: String = line.unwrap();
        v.push(l);
    }
    assert_ne!(0, v.len());

    /*
    let hm = get_head_movement(&v);
    assert_ne!(0, v.len());

    // Part 1
    if true {
        let tm = get_tail_movement(&hm);
        let mut unique_tm = tm.clone();
        unique_tm.sort();
        unique_tm.dedup();
        assert_eq!(6197, unique_tm.len());
        println!("Unique tail positions: {}", unique_tm.len());
    }

    // Part 2
    if true {
        let mut tails: Vec<Vec<(i32, i32)>> = Vec::new();
        let num_knots = 10;
        for i in 0..(num_knots - 1) {
            let my_head = if i == 0 {
                hm.clone()
            } else {
                tails[i - 1].clone()
            };
            let tm = get_tail_movement(&my_head);
            tails.push(tm);
        }

        let mut unique_tm = tails[tails.len() - 1].clone();
        unique_tm.sort();
        unique_tm.dedup();
        assert_eq!(2562, unique_tm.len());
        println!("Unique tail positions for part 2: {}", unique_tm.len());
    } */

    Ok(())
}
