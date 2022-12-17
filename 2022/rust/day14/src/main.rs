use std::{ io::{ self } };
//use std::array;
mod utils;
use std::cmp::Ordering;
use std::collections::HashMap;

/*
        const NEXT: [(usize, usize); 4] = [
            (1, 0),
            (usize::MAX, 0),
            (0, 1),
            (0, usize::MAX),
        ];
*/

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn array_compare_1() {
        let data_bytes = String::from(
            "5,0 -> 0,0
            498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9"
        );
        let vec_in: Vec<String> = utils::test_input_to_vec(data_bytes, true);
        assert_ne!(0, vec_in.len());

        let mut world: HashMap<(i32,i32), char> = HashMap::new();

        for line in vec_in {
            let xy: Vec<(i32, i32)> = line
                .split("->")
                .map(|l| l.trim())
                .map(|l2| {
                    let v: Vec<&str> = l2.split(",").collect();
                    let x = utils::robust_to_int(v[0]);
                    let y = utils::robust_to_int(v[1]);
                    return (x, y);
                })
                .collect();

            println!("len = {}", xy.len());

            for i in 0..xy.len()-1 {
                let i0 = xy[i+0];
                let i1 = xy[i+1];
                if i0.1 != i1.1 {
                    assert_eq!(i0.0, i1.0);
                    let y0 = if i0.1 < i1.1 {i0.1} else {i1.1};
                    let y1 = if i0.1 < i1.1 {i1.1} else {i0.1};
                    for y in y0..y1+1 {                    
                        world.insert((i0.0, y), '#');
                    }
                }
                else if i0.0 != i1.0 {
                    assert_eq!(i0.1, i1.1);
                    let x0 = if i0.0 < i1.0 {i0.0} else {i1.0};
                    let x1 = if i0.0 < i1.0 {i1.0} else {i0.0};
                    for x in x0..x1+1 {                    
                        world.insert((x, i0.1), '#');
                    }    
                }
                else {
                    assert!(false);
                }

            }

            //for item in sxy {
            //    world.insert(item.0.to_string(), '#');
            //}
        }
    }
}

pub fn main() -> io::Result<()> {
    let filename = if std::env::args().len() >= 2 {
        std::env::args().nth(1).unwrap()
    } else {
        String::from("input.txt")
    };
    let data_bytes = std::fs::read_to_string(filename).unwrap();

    Ok(())
}