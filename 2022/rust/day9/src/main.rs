use std::{
    cmp,
    collections::{HashMap, VecDeque},
    io::{self},
};
mod utils;

fn get_play_area(v:&Vec<String>)->(i32,i32){
    let mut w = 0;
    let mut h = 0;
    let mut x = 0;
    let mut y = 0;
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    for nl in v {
        let words:Vec<&str> = nl.split_whitespace().collect();
        if words.len() == 0 { break; }
        let dir = words[0];
        let mut xdir = 0;
        let mut ydir = 0;
        match dir.chars().nth(0).unwrap() {
            'R' => {
                xdir = 1;
            },
            'L' => {
                xdir = -1;
            },
            'U' => {
                ydir = 1;
            },
            _ => {
                ydir = -1;
            }
        }
        let spaces = words[1].parse::<i32>().unwrap();
        x += xdir * spaces;
        y += ydir * spaces;
        min_x = cmp::min(min_x, x);
        max_x = cmp::max(max_x, x);
        min_y = cmp::min(min_y, y);
        max_y = cmp::max(max_y, y);        

    }
    w = max_x - min_x;
    h = max_y - min_y;
    return (w,h);

}


fn get_head_movement(v:&Vec<String>)->Vec<(i32, i32)>{
    let mut x = 0;
    let mut y = 0;
    let mut head_movement: Vec<(i32, i32)> = Vec::new();
    for nl in v {
        let words:Vec<&str> = nl.split_whitespace().collect();
        if words.len() == 0 { break; }
        let dir = words[0];
        let mut xdir = 0;
        let mut ydir = 0;
        match dir.chars().nth(0).unwrap() {
            'R' => {
                xdir = 1;
            },
            'L' => {
                xdir = -1;
            },
            'U' => {
                ydir = 1;
            },
            _ => {
                ydir = -1;
            }
        }
        let spaces = words[1].parse::<i32>().unwrap();
        x += xdir * spaces;
        y += ydir * spaces;
        head_movement.push((x,y));
    }
    return head_movement;

}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn scenic_test() {
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

        let (w,h) = get_play_area(&v);

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
        
    let (w,h) = get_play_area(&v);
    println!("Play area = {}x{}", w, h);
    Ok(())
}
