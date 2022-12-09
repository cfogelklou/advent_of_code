use std::io::{self};
mod utils;

#[allow(dead_code)]
fn get_head_movement(v: &Vec<String>) -> Vec<(i32, i32)> {
    let mut x = 0;
    let mut y = 0;
    let mut head_movement: Vec<(i32, i32)> = Vec::new();
    head_movement.push((x, y));
    for nl in v {
        let words: Vec<&str> = nl.split_whitespace().collect();
        if words.len() == 0 {
            break;
        }
        let dir = words[0];
        let mut xdir = 0;
        let mut ydir = 0;
        match dir.chars().nth(0).unwrap() {
            'R' => {
                xdir = 1;
            }
            'L' => {
                xdir = -1;
            }
            'U' => {
                ydir = 1;
            }
            _ => {
                ydir = -1;
            }
        }
        let spaces = words[1].parse::<i32>().unwrap();
        for _i in 0..spaces {
            x += xdir;
            y += ydir;
            head_movement.push((x, y));
        }
    }
    return head_movement;
}


// Generates a vector that follows the head vector.
#[allow(dead_code)]
fn get_tail_movement(hm: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut tail_movement: Vec<(i32, i32)> = Vec::new();
    let mut x = 0;
    let mut y = 0;
    for head in hm {
        let (hx, hy) = head.clone();

        let dx = hx - x;
        let dy =  hy - y;

        if (dx).abs() < 2 && (dy).abs() < 2 {
            // Nothing to do
        }
        else {
            x += (dx).signum();
            y += (dy).signum();
 
        }
        tail_movement.push((x, y));

    }
    return tail_movement;
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn short_tail_test() {
        let raw_string: String = "R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2"
        .to_string();
        let v: Vec<String> = utils::test_input_to_vec(raw_string, true);
        assert_ne!(0, v.len());
        let hm = get_head_movement(&v);
        let tm = get_tail_movement(&hm);
        let mut unique_tm = tm.clone();
        unique_tm.sort();
        unique_tm.dedup();
        assert_eq!(13, unique_tm.len());
    }

    #[test]
    fn long_tail_test() {
        let raw_string: String = "R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20"
            .to_string();
        let v: Vec<String> = utils::test_input_to_vec(raw_string, true);
        assert_ne!(0, v.len());
        let hm = get_head_movement(&v);
        // Part 2
        {
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

            // Now show this.
            for time in 0..hm.len() {
                let mut matrix: [[char; 25]; 25] = [['.'; 25]; 25];
                // Go through each tail, choosing this timestamp
                for _tail_idx in 0..(num_knots) {
                    let tail_idx = num_knots - 1 - _tail_idx;
                    // If it is 0, then use the head, otherwise use the correct tail.
                    let points_vec_to_show = if tail_idx == 0 {
                        hm.clone()
                    } else {
                        tails[tail_idx - 1].clone()
                    };
                    // from this tail, show this timestamp
                    let (x, y) = points_vec_to_show[time];
                    // Update matrix
                    if x >= 0 && x < 25 && y >= 0 && y < 25 {
                        let char_to_show = if tail_idx == 0 { 'H' } else {(tail_idx).to_string().chars().nth(0).unwrap()}; 
                        matrix[(24 - y) as usize][x as usize] = char_to_show;
                            
                    }
                    // For breakpoint
                    print!("{}", 0);
                }
                print!("{}", 0);
            }

            let mut unique_tm = tails[tails.len() - 1].clone();
            unique_tm.sort();
            unique_tm.dedup();
            //assert_eq!(13, unique_tm.len());
            println!("Unique tail positions for part 2: {}", unique_tm.len());
        }
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

    let hm = get_head_movement(&v);
    assert_ne!(0, v.len());

    // Part 1
    if true {
        let tm = get_tail_movement(&hm);
        let mut unique_tm = tm.clone();
        unique_tm.sort();
        unique_tm.dedup();
        //assert_eq!(13, unique_tm.len());
        println!("Unique tail positions: {}", unique_tm.len());
    }

    // Part 2
    {
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
    }

    Ok(())
}
