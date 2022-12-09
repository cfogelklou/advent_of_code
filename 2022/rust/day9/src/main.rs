use std::{
    cmp,
    collections::{HashMap, VecDeque},
    io::{self},
};
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


#[allow(dead_code)]
fn get_tail_movement(hm: &Vec<(i32, i32)>, visualize: bool) -> Vec<(i32, i32)> {
    let mut tail_movement: Vec<(i32, i32)> = Vec::new();
    let mut x = 0;
    let mut y = 0;
    //tail_movement.push((x, y));
    for head in hm {
        let (hx, hy) = head.clone();

        if visualize {
            println!("before moving tail");
            draw_it((5, 5), head.clone(), (x, y));
        }

        let delta_x = hx - x;
        let delta_y =  hy - y;

        if (delta_x).abs() < 2 && (delta_y).abs() < 2 {
            // Nothing to do
        }
        else {
            x += (delta_x).signum();
            y += (delta_y).signum();
 
        }
        tail_movement.push((x, y));

    }
    return tail_movement;
}



#[allow(dead_code)]
fn draw_it(wh: (i32, i32), xy: (i32, i32), txy: (i32, i32)) {
    let (w, h) = wh;
    let (x, y) = xy;
    let (tx, ty) = txy;
    println!("for x,y:{},{}", xy.0, xy.1);
    for __y in 0..h {
        let _y = h - 1 - __y;
        let mut xystr: String = String::new();
        for _x in 0..w {
            let mut ch: String = String::from(".");
            if _x == x && _y == y {
                ch = String::from("H");
            } else if _x == tx && _y == ty {
                ch = String::from("T");
            }
            xystr.push_str(&ch);
        }
        println!("{}", xystr);
    }
}

#[allow(dead_code)]
fn get_hash_value_for_tree(x: i32, y: i32) -> String {
    let mut h: String;
    h = x.to_string();
    h.push_str(",");
    h.push_str(&y.to_string());
    return h;
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
        let tm = get_tail_movement(&hm, true);
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
                let tm = get_tail_movement(&my_head, true);
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
        let tm = get_tail_movement(&hm, false);
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
            let tm = get_tail_movement(&my_head, false);
            tails.push(tm);
        }

        let mut unique_tm = tails[tails.len() - 1].clone();
        unique_tm.sort();
        unique_tm.dedup();
        //assert_eq!(13, unique_tm.len());
        println!("Unique tail positions for part 2: {}", unique_tm.len());
    }

    Ok(())
}
